#!/usr/bin/env python3
"""Restart registration probe (spike astromech-wgtr): which crashes does Windows' restart
registration restart? Windows only; needs psutil (Pillow for screenshots).

    python scripts/one_process_probe.py --restart --app <exe> --endpoint <core.json> --out <dir> --config NAME

It reports, without gating, which crash kinds come back under one WER configuration (`--config
NAME`; `--clean-launch` starts the app with the default error mode and out of the runner's job):
abort, access_violation and main_thread_panic after 65 s, TerminateProcess after 65 s and an
abort after 10 s, each waited on for 90 s. Each configuration's run adds to the same
restart.json and restart.md. It drives any app that takes `--background` and a crash hook
argument (`--crash-hook-arg`, the spike's `--crash-hook`), publishes its control API in the
endpoint file (one JSON line `{token, cmd, args?}`, answered `{"ok": true, "value": V}` or
`{"ok": false, "error": E}`) and answers `ui_state` and `crash` (`{kind}`).

Restart mode is all that is left of the spike's probe: its other checks (windowless, open, close,
window crash, quit) run against the one app in scripts/webview_cost.py (astromech-xz7i). It runs
on no CI workflow now: astromech-fmub runs it at Sumeet's PC against the spike
(spikes/o1-one-process), and astromech-dl53's crash-restart job is to run it against the
installed app. Run it by hand on Windows after changing it.
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import socket
import subprocess
import sys
import time
from pathlib import Path


# --- The app ------------------------------------------------------------------------------------


class App:
    def __init__(self, a, psutil):
        self.exe = a.app.resolve()
        self.endpoint = a.endpoint
        self.log = a.log or a.endpoint.parent / "spike.log"
        self.ps = psutil
        self.proc: subprocess.Popen | None = None

    def start(self, args, flags=0) -> subprocess.Popen:
        return subprocess.Popen([str(self.exe), *args], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                                creationflags=flags)

    def launch(self, args, timeout=15.0, flags=0) -> dict:
        """Start the main instance and wait for its control port."""
        try:
            self.endpoint.unlink()
        except OSError:
            pass
        self.proc = self.start(args, flags)
        t0 = time.time()
        while time.time() - t0 < timeout:
            ep = self.read_endpoint()
            if ep and ep.get("pid") == self.proc.pid:
                ok, v = self.call("ui_state")
                if ok:
                    return {"ok": True, "s": round(time.time() - t0, 2)}
            if self.proc.poll() is not None:
                return {"ok": False, "error": f"exited with {self.proc.returncode} before its control port answered"}
            time.sleep(0.1)
        return {"ok": False, "error": f"no control port within {timeout} s"}

    def alive(self) -> bool:
        return self.proc is not None and self.proc.poll() is None

    def ours(self) -> list:
        """Every process running the app's exe."""
        out = []
        for p in self.ps.process_iter(["name", "exe"]):
            try:
                if (p.info["name"] or "").lower() == self.exe.name.lower():
                    out.append(p)
            except self.ps.Error:
                pass
        return out

    def kill_all(self) -> None:
        for p in self.ours():
            try:
                p.kill()
            except self.ps.Error:
                pass
        time.sleep(1)

    # control port

    def read_endpoint(self) -> dict | None:
        try:
            return json.loads(self.endpoint.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            return None

    def call(self, cmd: str, args=None, timeout=10.0) -> tuple[bool, object]:
        ep = self.read_endpoint()
        if not ep:
            return False, "no endpoint file"
        req = {"token": ep["token"], "cmd": cmd}
        if args is not None:
            req["args"] = args
        try:
            with socket.create_connection(("127.0.0.1", ep["port"]), timeout=2) as s:
                s.settimeout(timeout)
                s.sendall((json.dumps(req) + "\n").encode())
                buf = b""
                while not buf.endswith(b"\n"):
                    chunk = s.recv(65536)
                    if not chunk:
                        break
                    buf += chunk
            r = json.loads(buf.decode())
        except (OSError, ValueError) as e:
            return False, f"{cmd}: {e}"
        return (True, r.get("value")) if r.get("ok") else (False, r.get("error"))

    # log

    def lines(self, text: str, pid: int | None = None) -> list[str]:
        pid = pid or (self.proc.pid if self.proc else None)
        try:
            ls = self.log.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            return []
        # The spike's lines name their pid; the app's don't (one process writes its log).
        return [l for l in ls if text in l and (pid is None or not l.startswith("[") or l.startswith(f"[{pid} "))]


# --- Restart registration -----------------------------------------------------------------------


def wer_state(psutil) -> dict:
    """WerSvc and the Windows Error Reporting values, so 'no restart' can be told apart from
    'WER is off on the runner'."""
    import winreg

    out: dict = {}
    try:
        s = psutil.win_service_get("WerSvc").as_dict()
        out["WerSvc"] = {"status": s.get("status"), "start_type": s.get("start_type")}
    except Exception as e:
        out["WerSvc"] = f"unknown: {e}"
    keys = {
        "HKLM": (winreg.HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\Windows Error Reporting"),
        "HKCU": (winreg.HKEY_CURRENT_USER, r"SOFTWARE\Microsoft\Windows\Windows Error Reporting"),
        "HKLM policy": (winreg.HKEY_LOCAL_MACHINE, r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting"),
        "HKCU policy": (winreg.HKEY_CURRENT_USER, r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting"),
    }
    for name, (hive, path) in keys.items():
        vals = {}
        try:
            with winreg.OpenKey(hive, path) as k:
                for v in ("Disabled", "DontShowUI", "LoggingDisabled", "Consent", "DefaultConsent"):
                    try:
                        vals[v] = winreg.QueryValueEx(k, v)[0]
                    except OSError:
                        vals[v] = None
        except OSError:
            vals = "no key"
        out[name] = vals
    return out


def wer_reports() -> set[str]:
    out = set()
    for base in (os.environ.get("ProgramData", r"C:\ProgramData"), os.environ.get("LOCALAPPDATA", "")):
        for sub in ("ReportQueue", "ReportArchive"):
            d = Path(base) / "Microsoft" / "Windows" / "WER" / sub
            try:
                out |= {str(p) for p in d.iterdir()}
            except OSError:
                pass
    return out


def restarted(app: App, crashed_pid: int) -> list:
    out = []
    for p in app.ours():
        try:
            if p.pid != crashed_pid and "--restarted" in (p.cmdline() or []):
                out.append(p)
        except app.ps.Error:
            pass
    return out


CREATE_BREAKAWAY_FROM_JOB = 0x01000000
CREATE_DEFAULT_ERROR_MODE = 0x04000000


def wer_processes() -> list[str]:
    """WER's own processes running now (WerFault shows the crash UI and does the restart)."""
    import psutil

    out = []
    for p in psutil.process_iter(["name", "cmdline"]):
        try:
            if (p.info["name"] or "").lower() in ("werfault.exe", "wermgr.exe", "werfaultsecure.exe"):
                out.append(" ".join(p.info["cmdline"] or [p.info["name"]]))
        except psutil.Error:
            pass
    return out


def desktop_shot(path: Path) -> str | None:
    try:
        from PIL import ImageGrab

        ImageGrab.grab(all_screens=True).save(path)
        return path.name
    except Exception as e:  # a report, not a check
        return f"screenshot failed: {e}"


def restart_report(a, psutil) -> dict:
    """The five crashes under one WER configuration. A clean launch gives the app the default
    error mode and, if the runner's job allows it, takes it out of the job, as a start from
    Explorer or Start with Windows would be."""
    app = App(a, psutil)
    cases = [("abort", 65), ("access_violation", 65), ("main_thread_panic", 65), ("terminate", 65), ("abort", 10)]
    rep = {"config": a.config, "wer_before": wer_state(psutil), "cases": []}
    print(json.dumps(rep), flush=True)
    for kind, up in cases:
        app.kill_all()
        reports0 = wer_reports()
        case = {"kind": kind, "uptime_s": up}
        flags = CREATE_DEFAULT_ERROR_MODE | CREATE_BREAKAWAY_FROM_JOB if a.clean_launch else 0
        try:
            st = app.launch(["--background", a.crash_hook_arg], flags=flags)
        except OSError as e:
            # The job doesn't allow breakaway: keep the default error mode only.
            case["breakaway_refused"] = str(e)
            flags = CREATE_DEFAULT_ERROR_MODE
            st = app.launch(["--background", a.crash_hook_arg], flags=flags)
        case["launch"] = ("default error mode" if flags & CREATE_DEFAULT_ERROR_MODE else "inherited error mode") + (
            ", out of the job" if flags & CREATE_BREAKAWAY_FROM_JOB else "")
        if not st["ok"]:
            case["error"] = st
            rep["cases"].append(case)
            continue
        pid = app.proc.pid
        env = app.lines("restart: error mode", pid)
        case["app_saw"] = env[-1].split("restart: ", 1)[1] if env else None
        t_start = time.time() - st["s"]
        time.sleep(max(0.0, up - (time.time() - t_start)))
        if kind == "terminate":
            psutil.Process(pid).kill()  # TerminateProcess, as taskkill /F does
            case["sent"] = True
        else:
            ok, val = app.call("crash", {"kind": kind})
            case["sent"] = ok if ok else val
        try:
            code = app.proc.wait(timeout=30)
            case["exit_code"] = f"0x{code & 0xFFFFFFFF:08X}" if code is not None else None
        except subprocess.TimeoutExpired:
            case["exit_code"] = "still running after 30 s"
        t0 = time.time()
        wer_seen: set[str] = set()
        back: list = []
        while time.time() - t0 < 90:
            wer_seen |= set(wer_processes())
            if not case.get("screenshot") and time.time() - t0 >= 5:
                case["screenshot"] = desktop_shot(a.out / f"restart-{a.config}-{kind}-{up}s.png")
            back = restarted(app, pid)
            if back:
                break
            time.sleep(1)
        case["restarted"] = bool(back)
        case["restart_after_s"] = round(time.time() - t0, 1) if back else None
        if back:
            try:
                case["restart_cmdline"] = back[0].cmdline()
            except psutil.Error:
                pass
        case["wer_processes_seen"] = sorted(wer_seen)
        case["new_wer_reports"] = sorted(Path(p).name for p in wer_reports() - reports0)
        print(json.dumps(case), flush=True)
        rep["cases"].append(case)
        for p in back or []:
            try:
                p.kill()
            except psutil.Error:
                pass
        # A WER dialog left open for the dead process would hold up the next case.
        for p in psutil.process_iter(["name", "cmdline"]):
            try:
                if (p.info["name"] or "").lower() == "werfault.exe" and str(pid) in (p.info["cmdline"] or []):
                    p.kill()
            except psutil.Error:
                pass
    app.kill_all()
    rep["wer_after"] = wer_state(psutil)
    return rep


# --- Reports ------------------------------------------------------------------------------------


def git_commit() -> str:
    try:
        return os.environ.get("GITHUB_SHA") or subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    except Exception:
        return "unknown"


def header(title: str) -> list[str]:
    return [f"# {title}", "", f"{platform.system()} {platform.release()} ({platform.machine()}, {os.cpu_count()} CPUs), "
            f"commit `{git_commit()}`, {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime())}.", ""]


def restart_markdown(configs: list[dict]) -> str:
    lines = header("Restart registration (astromech-wgtr, reported, not gated)")
    lines += ["Registered with `RegisterApplicationRestart(\"--background --restarted\", RESTART_NO_PATCH | "
              "RESTART_NO_REBOOT)`. A restart counts when a process with `--restarted` appears within 90 s of the "
              "crash. Error mode 0x2 (SEM_NOGPFAULTERRORBOX) or job limit 0x400 (DIE_ON_UNHANDLED_EXCEPTION) keeps "
              "WER away from a crash.", ""]
    for r in configs:
        back = sorted({f"{c['kind']} at {c['uptime_s']} s" for c in r["cases"] if c.get("restarted")})
        verdict = (f"Windows restarted: {', '.join(back)}." if back
                   else "**Windows restarted none of the crashes.**")
        lines += [f"**{r['config']}**: {verdict}", "",
                  f"- WER before: `{json.dumps(r.get('wer_before'))}`",
                  f"- WER after: `{json.dumps(r.get('wer_after'))}`", ""]
    lines += ["| Config | Crash | Uptime | Launch | The app saw | Exit code | Restarted | After | WER processes | "
              "New WER reports |", "|---|---|---|---|---|---|---|---|---|---|"]
    for r in configs:
        for c in r["cases"]:
            if "error" in c:
                lines.append(f"| {r['config']} | {c['kind']} | {c['uptime_s']} s | {c.get('launch')} | | "
                             f"error: {c['error']} | | | | |")
                continue
            lines.append(f"| {r['config']} | {c['kind']} | {c['uptime_s']} s | {c.get('launch')} | "
                         f"{c.get('app_saw')} | {c.get('exit_code')} | {'yes' if c['restarted'] else 'no'} | "
                         f"{c.get('restart_after_s')} | {'; '.join(c.get('wer_processes_seen', [])) or 'none'} | "
                         f"{', '.join(c.get('new_wer_reports', [])) or 'none'} |")
    return "\n".join(lines) + "\n"


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--restart", action="store_true", required=True,
                   help="report restart registration (the only mode left; the other checks are webview_cost.py's)")
    p.add_argument("--app", type=Path, required=True, help="the app's exe")
    p.add_argument("--endpoint", type=Path, required=True, help="its core.json")
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--log", type=Path, default=None, help="its log (default: spike.log next to the endpoint)")
    p.add_argument("--crash-hook-arg", default="--crash-hook")
    p.add_argument("--config", default="runner", help="this WER configuration's name in the report")
    p.add_argument("--clean-launch", action="store_true",
                   help="start the app with the default error mode, out of the runner's job if allowed")
    a = p.parse_args(argv)
    # The Windows console is cp1252; the reports aren't.
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if os.name != "nt":
        print("one_process_probe.py runs on Windows only", file=sys.stderr)
        return 2
    import psutil

    a.out.mkdir(parents=True, exist_ok=True)
    # Each configuration's run adds to the same report, so they share one table.
    path = a.out / "restart.json"
    try:
        configs = [c for c in json.loads(path.read_text(encoding="utf-8")) if c["config"] != a.config]
    except (OSError, ValueError):
        configs = []
    configs.append(restart_report(a, psutil))
    path.write_text(json.dumps(configs, indent=1, default=str), encoding="utf-8")
    md = restart_markdown(configs)
    (a.out / "restart.md").write_text(md, encoding="utf-8")
    print(md)
    return 0


if __name__ == "__main__":
    sys.exit(main())
