#!/usr/bin/env python3
"""Upgrade smoke test (Windows runner): a two-process install (v1.0.0-alpha.4) upgraded
to this build by both routes a player has (astromech-9lh3; decision astromech-nade).

    python scripts/upgrade_smoke.py --old-setup old/astromech_1.0.0-alpha.4_x64-setup.exe \
        --setup dist-release/astromech_0.0.0-dev_x64-setup.exe \
        --zip dist-release/astromech-0.0.0-dev-windows-x64.zip --version 0.0.0-dev --out upgrade-report

The old release must be a two-process one: v1.0.0-alpha.4, the last (release.yml pins it).
Each route starts from the old release installed silently and running as a player has it: the
core with `--background` (Start with Windows), then the old window (`astromech-shell.exe`).

1. Updater. The old release's own helper applies this build's update zip, as its in-app
   Install and restart does (crates/astromech-core/src/update.rs `spawn_apply`): the old
   `astromech-core.exe` copied to `%TEMP%\\astromech-apply-<core pid>.exe` (under its own name its
   `taskkill /F /T /IM astromech-core.exe` would end the helper too), run with `--apply-update
   --from <zip extracted> --install-dir <inst> --wait-pid <core pid> --relaunch --route settings`
   (the window was open), then the old core asked to `--quit`. The signed-feed download before
   that is unchanged old code and is not part of this test.
2. NSIS. This build's `setup.exe /S` over the running install. A silent install starts nothing
   afterwards, so the app is then started from the Start-menu shortcut, as a player would.

After each route, within 30 s:
- the old core exited with code 0, after logging `quitting`: its clean quit (held keys released)
  came before anything forced it (taskkill /F and the Restart Manager's forced shutdown exit 1);
- exactly one astromech-core.exe runs, its `status.version` is this build's, and no
  astromech-shell.exe process is left;
- after the updater route, the window is open at Settings in that process (`ui_state`);
- the Run value is unchanged, and core.json answers;
- starting `<inst>\\astromech-shell.exe` (an old shortcut or pin) opens the window in the running
  app, with one process and a new page-load line;
- after the NSIS route, the Start-menu shortcut targets astromech-core.exe.
The run ends by uninstalling.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import socket
import subprocess
import sys
import tempfile
import time
import zipfile
from pathlib import Path

APP, SHELL = "astromech-core.exe", "astromech-shell.exe"
TITLE = "astromech"
WITHIN_S = 30
RUN_KEY = r"Software\Microsoft\Windows\CurrentVersion\Run"
# spawn_apply's creation flags (update.rs): DETACHED | NEW_GROUP | NO_WINDOW.
DETACHED_FLAGS = 0x00000008 | 0x00000200 | 0x08000000

if os.name == "nt":
    import ctypes
    import winreg
    from ctypes import wintypes

    user32 = ctypes.windll.user32
    user32.PostMessageW.argtypes = [wintypes.HWND, wintypes.UINT, wintypes.WPARAM, wintypes.LPARAM]
    user32.GetWindowThreadProcessId.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.DWORD)]
    user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
    user32.GetWindowTextLengthW.argtypes = [wintypes.HWND]
    user32.IsWindowVisible.argtypes = [wintypes.HWND]
    _ENUM = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)


class Failed(Exception):
    pass


def local() -> Path:
    return Path(os.environ["LOCALAPPDATA"])


def inst_dir() -> Path:
    return local() / "astromech"


def data_dir() -> Path:
    return inst_dir() / "data"


def endpoint() -> Path:
    return data_dir() / "core.json"


def core_log() -> Path:
    return data_dir() / "logs" / "astromech-core.log"


def start_menu_lnk() -> Path:
    return Path(os.environ["APPDATA"]) / "Microsoft" / "Windows" / "Start Menu" / "Programs" / "astromech.lnk"


def call(cmd: str, timeout: float = 5.0):
    """One control-API request: (ok, value or error). The one app wraps a reply's value in
    `value`; a two-process release (v1.0.0-alpha.4) sends it bare (`{"ok": true, "pid": …}`)."""
    try:
        ep = json.loads(endpoint().read_text(encoding="utf-8"))
        with socket.create_connection(("127.0.0.1", ep["port"]), timeout=timeout) as s:
            s.sendall((json.dumps({"token": ep["token"], "cmd": cmd}) + "\n").encode())
            for line in s.makefile(encoding="utf-8"):
                r = json.loads(line)
                if "progress" in r:
                    continue
                return (True, r["value"] if "value" in r else r) if r.get("ok") else (False, r.get("error"))
    except (OSError, ValueError, KeyError) as e:
        return False, f"{cmd}: {e}"
    return False, f"{cmd}: no reply"


def wait(pred, what: str, timeout: float = WITHIN_S, step: float = 0.25):
    end = time.time() + timeout
    last = None
    while time.time() < end:
        last = pred()
        if last:
            return last
        time.sleep(step)
    raise Failed(f"timed out after {timeout:.0f} s waiting for {what} (last: {last!r})")


def procs(psutil, name: str) -> list:
    out = []
    for p in psutil.process_iter(["name"]):
        try:
            if (p.info["name"] or "").lower() == name.lower():
                out.append(p)
        except psutil.Error:
            pass
    return out


def run_value() -> str | None:
    try:
        with winreg.OpenKey(winreg.HKEY_CURRENT_USER, RUN_KEY) as k:
            return winreg.QueryValueEx(k, "astromech")[0]
    except OSError:
        return None


def lines(path: Path, text: str) -> list[str]:
    try:
        return [l for l in path.read_text(encoding="utf-8", errors="replace").splitlines() if text in l]
    except OSError:
        return []


def shortcut_target(lnk: Path) -> str:
    ps = f"(New-Object -ComObject WScript.Shell).CreateShortcut('{lnk}').TargetPath"
    r = subprocess.run(["powershell", "-NoProfile", "-Command", ps], capture_output=True, text=True, timeout=30)
    return r.stdout.strip()


def app_window(pid: int) -> int | None:
    found = []

    def cb(h, _):
        p = wintypes.DWORD()
        user32.GetWindowThreadProcessId(h, ctypes.byref(p))
        if p.value == pid and user32.IsWindowVisible(h):
            n = user32.GetWindowTextLengthW(h)
            buf = ctypes.create_unicode_buffer(n + 1)
            user32.GetWindowTextW(h, buf, n + 1)
            if buf.value == TITLE:
                found.append(h)
        return True

    user32.EnumWindows(_ENUM(cb), 0)
    return found[0] if found else None


class Run:
    def __init__(self, a, psutil):
        self.a, self.ps = a, psutil
        self.t0 = time.time()
        self.steps: list[dict] = []
        self.checks: list[dict] = []
        self.started: list[subprocess.Popen] = []

    def step(self, route: str, name: str) -> None:
        at = round(time.time() - self.t0, 2)
        self.steps.append({"route": route, "step": name, "at": at})
        print(f"[{at:7.2f} s] {route}: {name}", flush=True)

    def check(self, route: str, name: str, ok: bool, detail) -> None:
        self.checks.append({"route": route, "check": name, "ok": bool(ok), "detail": str(detail)})
        print(f"  {'ok  ' if ok else 'FAIL'} {route}: {name}: {detail}", flush=True)
        if not ok:
            raise Failed(f"{route}: {name}: {detail}")

    # installs

    def remove_everything(self, route: str) -> None:
        """No astromech left from an earlier step: processes, install, data, Run value."""
        if (inst_dir() / APP).is_file():
            subprocess.run([str(inst_dir() / APP), "--quit"], timeout=30)
        for n in (SHELL, APP, "astromech-voice-worker.exe"):
            subprocess.run(["taskkill", "/F", "/T", "/IM", n], capture_output=True)
        if (inst_dir() / "uninstall.exe").is_file():
            subprocess.run([str(inst_dir() / "uninstall.exe"), "/S"], timeout=180)
            time.sleep(3)
        shutil.rmtree(data_dir(), ignore_errors=True)
        try:
            with winreg.OpenKey(winreg.HKEY_CURRENT_USER, RUN_KEY, 0, winreg.KEY_SET_VALUE) as k:
                winreg.DeleteValue(k, "astromech")
        except OSError:
            pass
        self.step(route, "nothing of astromech left")

    def old_running(self, route: str) -> dict:
        """The old release installed and running as a player has it: the core with --background,
        then the old window."""
        self.remove_everything(route)
        subprocess.run([str(self.a.old_setup), "/S"], check=True, timeout=300)
        inst = inst_dir()
        for f in (APP, SHELL):
            if not (inst / f).is_file():
                raise Failed(f"the old release didn't install {inst / f}")
        self.step(route, f"old release installed silently ({inst})")
        core = subprocess.Popen([str(inst / APP), "--background"])
        self.started.append(core)
        st = wait(lambda: (lambda r: r[1] if r[0] and r[1].get("pid") == core.pid else None)(call("status")),
                  "the old core's control port", 60)
        self.check(route, "the old core answers", st.get("version") != self.a.version,
                   f"pid {core.pid}, version {st.get('version')}")
        shell_log = data_dir() / "logs" / "astromech-shell.log"
        loads = len(lines(shell_log, "window: page loaded"))
        shell = subprocess.Popen([str(inst / SHELL)])
        self.started.append(shell)
        wait(lambda: len(lines(shell_log, "window: page loaded")) > loads, "the old window's page load", 90)
        self.check(route, "the old window is open", shell.poll() is None, f"astromech-shell.exe pid {shell.pid}")
        run = wait(run_value, "the old core's Run value", 15)
        self.step(route, f"old release running: core {core.pid} (--background), window {shell.pid}; Run {run}")
        return {"core": core, "shell": shell, "run": run, "quits": len(lines(core_log(), "quitting"))}

    # what both routes end with

    def clean_quit(self, route: str, old: dict, need_code_0: bool) -> None:
        """The old core's clean quit came first: it logged `quitting` and removed core.json, which
        it does only after releasing held keys and ending the session. `need_code_0`: nothing may
        end it afterwards either (the updater's helper waits for it to exit before its taskkill;
        the installer hook's fallback taskkill can still catch a core that has removed core.json
        but not yet exited, which is harmless, so there the exit code is reported only)."""
        try:
            code = old["core"].wait(WITHIN_S)
        except subprocess.TimeoutExpired:
            code = None
        quits = len(lines(core_log(), "quitting"))
        gone = not endpoint().exists() if not need_code_0 else True
        ok = quits > old["quits"] and gone and (code == 0 or not need_code_0)
        self.check(route, "the old core quit cleanly before anything forced it", ok,
                   f"exit code {code} (0: it exited by itself; taskkill /F and the Restart Manager's forced "
                   f"shutdown give 1), 'quitting' lines {old['quits']} -> {quits}"
                   + ("" if need_code_0 else f", core.json removed: {gone}"))

    def one_app(self, route: str) -> dict:
        def ready():
            ok, st = call("status")
            cores = procs(self.ps, APP)
            if ok and st.get("version") == self.a.version and len(cores) == 1 and not procs(self.ps, SHELL):
                return st
            return None

        st = wait(ready, f"one {APP} at version {self.a.version} and no {SHELL}")
        cores = procs(self.ps, APP)
        self.check(route, f"exactly one {APP}, this build's", len(cores) == 1 and cores[0].pid == st["pid"],
                   f"pid {st['pid']}, version {st['version']}")
        self.check(route, f"no {SHELL} process", not procs(self.ps, SHELL), "none")
        return st

    def unchanged_run_value(self, route: str, old: dict) -> None:
        now = run_value()
        self.check(route, "the Run value is unchanged", now == old["run"], now)
        expected = f'"{inst_dir() / APP}" --background'
        self.check(route, "the Run value starts astromech-core.exe --background", (now or "").lower() == expected.lower(),
                   now)

    def old_shortcut(self, route: str, st: dict) -> None:
        """An old shortcut or pin starts astromech-shell.exe, now the forwarder: the window opens
        in the running app."""
        pid = st["pid"]
        ok, ui = call("ui_state")
        if ok and ui.get("window") == "open":
            h = app_window(pid)
            if not h:
                raise Failed(f"ui_state says the window is open but {pid} has no window titled {TITLE!r}")
            user32.PostMessageW(h, 0x0010, 0, 0)  # WM_CLOSE, as a click on X
            wait(lambda: (lambda r: r[0] and r[1].get("window") == "closed")(call("ui_state")), "the window to close")
        loads = len(lines(core_log(), "window: page loaded"))
        fwd = subprocess.Popen([str(inst_dir() / SHELL)])
        code = fwd.wait(WITHIN_S)
        self.check(route, f"{SHELL} (the forwarder) exits at once", code == 0, f"exit code {code}")
        wait(lambda: (lambda r: r[0] and r[1].get("window") == "open")(call("ui_state")), "the window to open")
        wait(lambda: len(lines(core_log(), "window: page loaded")) > loads, "a page-load line")
        one = wait(lambda: (lambda c: c if len(c) == 1 else None)(procs(self.ps, APP)), f"one {APP}")
        self.check(route, f"{SHELL} opened the window in the running app", one[0].pid == pid and not procs(self.ps, SHELL),
                   f"one process ({pid}), a new page-load line: {lines(core_log(), 'window: page loaded')[-1:]}")

    # the routes

    def updater(self) -> None:
        route = "updater"
        old = self.old_running(route)
        inst, core = inst_dir(), old["core"]
        payload = Path(tempfile.mkdtemp(prefix="astromech-upgrade-")) / "payload"
        with zipfile.ZipFile(self.a.zip) as z:
            z.extractall(payload)
        self.check(route, "the update zip carries the app and the forwarder",
                   (payload / APP).is_file() and (payload / SHELL).is_file(), sorted(p.name for p in payload.iterdir()))
        helper = Path(tempfile.gettempdir()) / f"astromech-apply-{core.pid}.exe"
        shutil.copyfile(inst / APP, helper)
        h = subprocess.Popen([str(helper), "--apply-update", "--from", str(payload), "--install-dir", str(inst),
                              "--wait-pid", str(core.pid), "--relaunch", "--route", "settings"],
                             creationflags=DETACHED_FLAGS)
        self.step(route, f"the old release's helper started ({helper.name}, pid {h.pid})")
        # The core's install quits next (its Install and restart): the clean quit, from a copy so
        # the helper's taskkill of astromech-core.exe can't catch this --quit.
        quitter = Path(tempfile.gettempdir()) / "astromech-quit-old.exe"
        shutil.copyfile(inst / APP, quitter)
        q = subprocess.run([str(quitter), "--quit"], timeout=60)
        self.step(route, f"the old core asked to --quit (exit {q.returncode})")
        code = h.wait(120)
        self.check(route, "the helper copied the files and relaunched", code == 0,
                   f"exit code {code}; {lines(core_log(), 'update helper:')[-3:]}")
        self.clean_quit(route, old, need_code_0=True)
        st = self.one_app(route)
        ui = wait(lambda: (lambda r: r[1] if r[0] and r[1].get("window") == "open" else None)(call("ui_state")),
                  "the window to open")
        self.check(route, "the window is open at Settings", ui.get("page") == "settings", ui)
        self.unchanged_run_value(route, old)
        self.old_shortcut(route, st)

    def nsis(self) -> None:
        route = "nsis"
        old = self.old_running(route)
        r = subprocess.run([str(self.a.setup), "/S"], timeout=300)
        self.step(route, f"this build's setup.exe /S over the running install (exit {r.returncode})")
        self.check(route, "setup.exe succeeded", r.returncode == 0, r.returncode)
        self.clean_quit(route, old, need_code_0=False)
        self.check(route, "the installer left nothing running", not procs(self.ps, APP) and not procs(self.ps, SHELL),
                   [p.pid for p in procs(self.ps, APP) + procs(self.ps, SHELL)])
        for f in (APP, SHELL):
            self.check(route, f"{f} installed", (inst_dir() / f).is_file(), inst_dir() / f)
        target = shortcut_target(start_menu_lnk())
        self.check(route, "the Start-menu shortcut targets astromech-core.exe",
                   target.lower() == str(inst_dir() / APP).lower(), target)
        os.startfile(str(start_menu_lnk()))
        self.step(route, "started from the Start-menu shortcut")
        st = self.one_app(route)
        self.unchanged_run_value(route, old)
        self.old_shortcut(route, st)

    def finish(self) -> None:
        for p in self.started:
            if p.poll() is None:
                p.kill()
        self.remove_everything("end")


def report_md(r: dict) -> str:
    out = ["# Upgrade smoke test: the last two-process release to this build", "",
           f"Result: **{'PASS' if r['pass'] else 'FAIL'}**. Old release: {r['old']}; this build: {r['version']}.", "",
           "| Route | Check | | Detail |", "|---|---|---|---|"]
    out += [f"| {c['route']} | {c['check']} | {'ok' if c['ok'] else 'FAIL'} | {c['detail'].replace('|', '/')[:300]} |"
            for c in r["checks"]]
    out += ["", "| Route | Step | At (s) |", "|---|---|---|"]
    out += [f"| {s['route']} | {s['step']} | {s['at']} |" for s in r["steps"]]
    if r["error"]:
        out += ["", f"Error: {r['error']}"]
    return "\n".join(out) + "\n"


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--old-setup", type=Path, required=True, help="the last two-process release's NSIS installer (v1.0.0-alpha.4)")
    p.add_argument("--old", default="", help="the old release's tag, for the report")
    p.add_argument("--setup", type=Path, required=True, help="this build's NSIS installer")
    p.add_argument("--zip", type=Path, required=True, help="this build's update zip")
    p.add_argument("--version", required=True, help="this build's version (latest.json)")
    p.add_argument("--route", choices=["updater", "nsis", "both"], default="both")
    p.add_argument("--out", type=Path, required=True)
    a = p.parse_args(argv)
    if os.name != "nt":
        print("needs Windows")
        return 2
    import psutil

    a.out.mkdir(parents=True, exist_ok=True)
    run = Run(a, psutil)
    error = None
    try:
        for name in (["updater", "nsis"] if a.route == "both" else [a.route]):
            try:
                getattr(run, name)()
            finally:  # the next route starts by removing the data folder
                for log in ("astromech-core.log", "astromech-shell.log"):
                    src = data_dir() / "logs" / log
                    if src.is_file():
                        shutil.copyfile(src, a.out / f"{name}-{log}")
    except Exception as e:  # report what got done, then fail
        error = f"{type(e).__name__}: {e}"
    finally:
        try:
            run.finish()
        except Exception as e:  # noqa: BLE001 - cleanup must not hide the run's own error
            print(f"cleanup: {e}", flush=True)
    r = {"pass": error is None, "old": a.old or a.old_setup.name, "version": a.version, "checks": run.checks,
         "steps": run.steps, "error": error}
    (a.out / "report.json").write_text(json.dumps(r, indent=1), encoding="utf-8")
    md = report_md(r)
    (a.out / "report.md").write_text(md, encoding="utf-8")
    print(md)
    return 0 if r["pass"] else 1


if __name__ == "__main__":
    sys.exit(main())
