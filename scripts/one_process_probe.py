#!/usr/bin/env python3
"""One-process probe (Atelier astromech-wgtr): can one Tauri process hold the tray with no
window, survive the window crashing, free WebView2 when the window closes, and leave the app's
own Raw Input listener receiving keys? Windows only; needs psutil (Pillow for screenshots).

    python scripts/one_process_probe.py --app <exe> --endpoint <core.json> --out <dir>
    python scripts/one_process_probe.py --app <exe> --endpoint <core.json> --out <dir> --restart

It drives any app with this interface (the spike in spikes/o1-one-process now, the real app
later): `--background` starts it with no window; a second start with `--route X` hands over to
the running one and exits 0; the endpoint file names a localhost port that takes one JSON line
`{token, cmd, args?}` and answers `{"ok": true, "value": V}` or `{"ok": false, "error": E}`, with
the commands `ui_state` (`{window, tray, sink_keys, ...}`), `open_window`, `tray_churn`, `quit`
and, with the crash hook on, `crash` (`{kind}`); and the log has a
`window: page loaded N ms after start` line per page load, prefixed `[<pid> ...`.

A WebView2 process belongs to the app when it descends from the app, or its command line has
`--webview-exe-name=<the app's exe name>`, or it names the app's EBWebView user-data folder, so a
renderer orphaned by a dead parent still counts.

The default run asserts checks 1-7 of astromech-wgtr (exit status 1 if one fails) and reports,
without gating, memory and CPU (decision astromech-01ft), the Raw Input answer with the listener
started before Tauri's build, and what a crashed window showed. `--restart` reports, without
gating, which crash kinds Windows' restart registration restarts.
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import shlex
import socket
import statistics
import subprocess
import sys
import time
from pathlib import Path

WEBVIEW = "msedgewebview2.exe"
SCAN_F13 = 0x64  # Set-1 scan code: a key nothing on the runner reacts to
ROUTE = "keybinds"


# --- Win32 --------------------------------------------------------------------------------------

if os.name == "nt":
    import ctypes
    from ctypes import wintypes

    class _KEYBDINPUT(ctypes.Structure):
        _fields_ = [("wVk", wintypes.WORD), ("wScan", wintypes.WORD), ("dwFlags", wintypes.DWORD),
                    ("time", wintypes.DWORD), ("dwExtraInfo", ctypes.c_size_t)]

    class _MOUSEINPUT(ctypes.Structure):
        _fields_ = [("dx", wintypes.LONG), ("dy", wintypes.LONG), ("mouseData", wintypes.DWORD),
                    ("dwFlags", wintypes.DWORD), ("time", wintypes.DWORD), ("dwExtraInfo", ctypes.c_size_t)]

    class _U(ctypes.Union):
        _fields_ = [("ki", _KEYBDINPUT), ("mi", _MOUSEINPUT)]

    class _INPUT(ctypes.Structure):
        _fields_ = [("type", wintypes.DWORD), ("u", _U)]

    user32 = ctypes.WinDLL("user32", use_last_error=True)
    user32.SendInput.argtypes = [wintypes.UINT, ctypes.c_void_p, ctypes.c_int]
    user32.PostMessageW.argtypes = [wintypes.HWND, wintypes.UINT, wintypes.WPARAM, wintypes.LPARAM]
    user32.GetWindowThreadProcessId.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.DWORD)]
    user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
    user32.GetWindowTextLengthW.argtypes = [wintypes.HWND]
    user32.IsWindowVisible.argtypes = [wintypes.HWND]
    user32.GetWindowRect.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.RECT)]
    user32.SetForegroundWindow.argtypes = [wintypes.HWND]
    _ENUM = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)


def tap_key() -> str | None:
    """Press and release F13 through SendInput; an error string if Windows refused."""
    for up in (False, True):
        i = _INPUT(type=1)
        i.u.ki = _KEYBDINPUT(0, SCAN_F13, 0x8 | (0x2 if up else 0), 0, 0)
        if user32.SendInput(1, ctypes.byref(i), ctypes.sizeof(_INPUT)) != 1:
            return f"SendInput failed, error {ctypes.get_last_error()}"
        time.sleep(0.05)
    return None


def windows_of(pid: int) -> list[tuple[int, str, bool]]:
    """Top-level windows of `pid`: (hwnd, title, visible)."""
    out = []

    def cb(h, _):
        p = wintypes.DWORD()
        user32.GetWindowThreadProcessId(h, ctypes.byref(p))
        if p.value == pid:
            n = user32.GetWindowTextLengthW(h)
            buf = ctypes.create_unicode_buffer(n + 1)
            user32.GetWindowTextW(h, buf, n + 1)
            out.append((h, buf.value, bool(user32.IsWindowVisible(h))))
        return True

    user32.EnumWindows(_ENUM(cb), 0)
    return out


def find_window(pid: int, title: str) -> int | None:
    for h, t, _ in windows_of(pid):
        if t == title:
            return h
    return None


def screenshot(hwnd: int, path: Path) -> str:
    try:
        from PIL import ImageGrab
    except ImportError:
        return "no Pillow"
    try:
        user32.SetForegroundWindow(hwnd)
        time.sleep(0.5)
        r = wintypes.RECT()
        user32.GetWindowRect(hwnd, ctypes.byref(r))
        ImageGrab.grab(bbox=(r.left, r.top, r.right, r.bottom), all_screens=True).save(path)
        return path.name
    except Exception as e:  # a report, not a check
        return f"screenshot failed: {e}"


# --- The app ------------------------------------------------------------------------------------


class App:
    def __init__(self, a, psutil):
        self.exe = a.app.resolve()
        self.endpoint = a.endpoint
        self.log = a.log or a.endpoint.parent / "spike.log"
        self.user_data = str(a.user_data or a.endpoint.parent / "EBWebView").lower().replace("/", "\\")
        self.title = a.title
        self.ps = psutil
        self.proc: subprocess.Popen | None = None

    # processes

    def start(self, args) -> subprocess.Popen:
        return subprocess.Popen([str(self.exe), *args], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    def launch(self, args, timeout=15.0) -> dict:
        """Start the main instance and wait for its control port."""
        try:
            self.endpoint.unlink()
        except OSError:
            pass
        self.proc = self.start(args)
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
        for w in self.webviews(include_orphans=True):
            try:
                self.ps.Process(w["pid"]).kill()
            except self.ps.Error:
                pass
        time.sleep(1)

    def webviews(self, include_orphans=True) -> list[dict]:
        """WebView2 processes that belong to the app, and why each does."""
        kids = set()
        if self.alive():
            try:
                kids = {c.pid for c in self.ps.Process(self.proc.pid).children(recursive=True)}
            except self.ps.Error:
                pass
        flag = f"--webview-exe-name={self.exe.name.lower()}"
        out = []
        for p in self.ps.process_iter(["name", "cmdline"]):
            try:
                if (p.info["name"] or "").lower() != WEBVIEW:
                    continue
                cmd = " ".join(p.info["cmdline"] or []).lower()
                why = []
                if p.pid in kids:
                    why.append("descendant")
                if include_orphans and flag in cmd:
                    why.append("webview-exe-name")
                if include_orphans and self.user_data in cmd.replace("/", "\\"):
                    why.append("user-data")
                if why:
                    kind = next((a.split("=", 1)[1] for a in (p.info["cmdline"] or []) if a.startswith("--type=")),
                                "browser")
                    out.append({"pid": p.pid, "type": kind, "why": why})
            except self.ps.Error:
                pass
        return out

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

    def state(self) -> dict | None:
        ok, v = self.call("ui_state")
        return v if ok else None

    # log

    def lines(self, text: str, pid: int | None = None) -> list[str]:
        pid = pid or (self.proc.pid if self.proc else None)
        try:
            ls = self.log.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            return []
        return [l for l in ls if text in l and (pid is None or l.startswith(f"[{pid} "))]

    def page_loads(self) -> int:
        return len(self.lines("window: page loaded"))


def tray_ok(v: dict | None) -> tuple[bool, str]:
    """The tray is there: TrayIcon::rect() has a rectangle, or (no notification area in the
    session) the icon was built and the main-thread round trip answered with no rectangle."""
    if not v:
        return False, "ui_state didn't answer"
    if v.get("tray"):
        return True, "rect"
    if v.get("tray_built") and "Ok(None)" in str(v.get("tray_rect")):
        return True, "fallback: icon built and the main thread answered, but no rect"
    return False, f"tray_rect {v.get('tray_rect')}"


def wait(pred, timeout: float, step=0.2):
    """Poll `pred` until it returns something truthy; that value, or the last one."""
    t0 = time.time()
    v = pred()
    while not v and time.time() - t0 < timeout:
        time.sleep(step)
        v = pred()
    return v


def measure(app: App, seconds: float) -> dict:
    """Private working set (USS) and CPU of the app and its WebView2 processes, sampled each second."""
    ps = app.ps
    first, last, mems, counts, seen = {}, {}, [], [], set()
    t0 = time.time()
    while True:
        procs = [app.proc.pid] if app.alive() else []
        wv = app.webviews()
        procs += [w["pid"] for w in wv]
        seen |= {w["pid"] for w in wv}
        total = 0
        for pid in procs:
            try:
                p = ps.Process(pid)
                total += p.memory_full_info().uss
                c = p.cpu_times()
                cpu = c.user + c.system
                first.setdefault(pid, cpu)
                last[pid] = cpu
            except ps.Error:
                pass
        mems.append(total / 2**20)
        counts.append(len(procs))
        if time.time() - t0 >= seconds:
            break
        time.sleep(1)
    wall = time.time() - t0
    used = sum(last[p] - first[p] for p in last)
    return {
        "seconds": round(wall, 1),
        "uss_mb_mean": round(statistics.mean(mems), 1),
        "uss_mb_max": round(max(mems), 1),
        "cpu_core_pct": round(100 * used / wall, 2),
        "cpu_machine_pct": round(100 * used / wall / (os.cpu_count() or 1), 2),
        "processes_max": max(counts),
        "webview_pids_seen": sorted(seen),
    }


# --- The checks ---------------------------------------------------------------------------------


class Probe:
    def __init__(self, a, psutil):
        self.a = a
        self.app = App(a, psutil)
        self.checks: dict[str, dict] = {}
        self.cost: dict[str, dict] = {}
        self.notes: dict[str, object] = {}
        self.sink: list[dict] = []

    def check(self, cid: str, name: str, ok: bool, detail) -> bool:
        self.checks[cid] = {"name": name, "pass": bool(ok), "detail": detail}
        print(f"{'PASS' if ok else 'FAIL'} {cid} {name}: {json.dumps(detail, default=str)}", flush=True)
        return ok

    def sink_check(self, label: str, into: list) -> bool:
        """An F-key through SendInput raises the listener's count."""
        v0 = self.app.state()
        if v0 is None:
            into.append({"when": label, "pass": False, "error": "ui_state didn't answer"})
            return False
        err = tap_key()
        v1 = wait(lambda: (v := self.app.state()) and v["sink_keys"] > v0["sink_keys"] and v, 3.0)
        after = v1["sink_keys"] if v1 else (self.app.state() or {}).get("sink_keys")
        ok = err is None and bool(v1)
        into.append({"when": label, "before": v0["sink_keys"], "after": after, "pass": ok, "send_error": err})
        return ok

    def second_start(self, route: str) -> dict:
        """A second start with `--route`: it exits 0 and the window loads a page within 10 s."""
        loads = self.app.page_loads()
        t0 = time.time()
        q = self.app.start(["--route", route])
        try:
            code = q.wait(timeout=15)
        except subprocess.TimeoutExpired:
            q.kill()
            code = "timeout"
        loaded = wait(lambda: self.app.page_loads() > loads, 10.0)
        return {"exit_code": code, "page_loaded": bool(loaded), "s": round(time.time() - t0, 2)}

    def close_window(self) -> dict:
        """WM_CLOSE to the window found by its title, as a click on X; then the window, WebView2
        and the tray."""
        hwnd = find_window(self.app.proc.pid, self.app.title)
        if not hwnd:
            return {"pass": False, "error": "no window with the title"}
        t0 = time.time()
        user32.PostMessageW(hwnd, 0x0010, 0, 0)
        closed = wait(lambda: (v := self.app.state()) and v["window"] == "closed", 10.0)
        t_closed = round(time.time() - t0, 2)
        gone = wait(lambda: not self.app.webviews(), 15.0, 0.5)
        left = self.app.webviews()
        ok_tray, how = tray_ok(self.app.state())
        r = {"window_closed_s": t_closed if closed else None, "webview_gone_s": round(time.time() - t0, 2) if gone
             else None, "webview_left": left, "app_alive": self.app.alive(), "tray": how}
        r["pass"] = bool(closed) and bool(gone) and self.app.alive() and ok_tray
        return r

    def run(self) -> None:
        a, app = self.a, self.app
        start_args = shlex.split(a.start_args)
        app.kill_all()

        # 1. Windowless.
        st = app.launch(["--background", *start_args])
        if not st["ok"]:
            self.check("1", "windowless", False, st)
            return
        tray = wait(lambda: tray_ok(app.state())[0], 10.0)
        v = app.state()
        after_build: list = []
        self.sink_check("at start", after_build)
        m = measure(app, a.idle_seconds)
        self.cost["background"] = m
        self.check("1", "windowless", bool(tray) and (v or {}).get("window") == "closed" and not m["webview_pids_seen"],
                   {"started_s": st["s"], "tray": tray_ok(v)[1], "window": v and v["window"],
                    "webview_pids_in_30s": m["webview_pids_seen"]})

        # 4. Open, by a second start.
        r = self.second_start(ROUTE)
        wv = wait(lambda: app.webviews(), 10.0)
        self.check("4", "open", r["exit_code"] == 0 and r["page_loaded"] and bool(wv),
                   {**r, "webview": wv})
        self.sink_check("after the window opens", after_build)
        time.sleep(5)
        self.cost["window open"] = measure(app, a.idle_seconds)

        # 5. Close, over 5 open and close cycles.
        cycles = []
        for i in range(5):
            if i:
                o = self.second_start(ROUTE)
                if not o["page_loaded"]:
                    cycles.append({"cycle": i + 1, "pass": False, "error": "reopen didn't load", **o})
                    continue
                time.sleep(2)
            c = self.close_window()
            cycles.append({"cycle": i + 1, **c})
            if i == 0:
                self.sink_check("after the window closes", after_build)
            self.cost[f"after close {i + 1}"] = measure(app, a.idle_seconds)
        self.check("5", "close", all(c["pass"] for c in cycles), cycles)

        # 3. Tray churn from a worker thread.
        ok, val = app.call("tray_churn", timeout=60)
        after = app.state()
        self.check("3", "tray churn", ok and after is not None, {"reply": val, "ui_state_after": after})
        self.sink_check("after tray_churn", after_build)
        self.check("2", "Raw Input survives Tauri (listener after build)",
                   all(s["pass"] for s in after_build) and len(after_build) == 4, after_build)

        # 6. Window crash: the renderer, then the browser process.
        crashes = []
        for target in ("renderer", "browser"):
            crashes.append(self.window_crash(target))
        self.notes["window crash"] = crashes
        self.check("6", "window crash", all(c["pass"] for c in crashes),
                   [{k: c[k] for k in ("target", "pass", "app_alive", "tray", "control", "reopened")} for c in crashes])

        # 7. Quit from a worker thread, with the window open.
        if (app.state() or {}).get("window") == "closed":
            self.second_start(ROUTE)
            time.sleep(2)
        ok, val = app.call("quit")
        t0 = time.time()
        try:
            code = app.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            code = None
        exited_s = round(time.time() - t0, 2)
        gone = wait(lambda: not app.webviews(), 5.0, 0.5)
        self.check("7", "quit", ok and code == 0 and bool(gone),
                   {"reply": val, "exit_code": code, "exited_s": exited_s, "webview_left": app.webviews()})
        app.kill_all()

    def window_crash(self, target: str) -> dict:
        app = self.app
        r = {"target": target}
        if (app.state() or {}).get("window") == "closed":
            r["open"] = self.second_start(ROUTE)
        time.sleep(3)
        v0 = app.state() or {}
        procs = app.webviews()
        victims = [w for w in procs if (w["type"] == "renderer" if target == "renderer" else w["type"] == "browser")]
        r["killed"] = victims
        if not victims:
            r.update({"pass": False, "error": f"no {target} process belongs to the app", "webview": procs,
                      "app_alive": app.alive(), "tray": None, "control": None, "reopened": None})
            return r
        failed_before = len(app.lines("process failed"))
        for w in victims:
            try:
                app.ps.Process(w["pid"]).kill()
            except app.ps.Error:
                pass
        time.sleep(6)
        v1 = app.state()
        ok_tray, how = tray_ok(v1)
        r["app_alive"] = app.alive()
        r["control"] = v1 is not None
        r["tray"] = how
        r["window_after_kill"] = v1 and v1["window"]
        r["page_beats_before"] = v0.get("page_beats")
        r["page_beats_6s_after"] = v1 and v1.get("page_beats")
        r["page_alive_after_kill"] = bool(v1 and v0.get("page_beats") is not None
                                          and v1.get("last_beat_ms_ago") is not None
                                          and v1["last_beat_ms_ago"] < 2000)
        r["webview_after_kill"] = app.webviews()
        r["process_failed_log"] = app.lines("process failed")[failed_before:]
        hwnd = find_window(app.proc.pid, app.title) if app.alive() else None
        if hwnd:
            r["screenshot"] = screenshot(hwnd, self.a.out / f"dead-window-{target}.png")
        # Closing and reopening loads the page again.
        if hwnd:
            r["close"] = self.close_window()
        o = self.second_start(ROUTE)
        time.sleep(3)
        v2 = app.state() or {}
        r["reopen"] = o
        r["reopened"] = o["page_loaded"]
        r["page_beats_after_reopen"] = v2.get("page_beats")
        r["pass"] = r["app_alive"] and ok_tray and r["control"] and r["reopened"]
        return r

    def before_build(self) -> None:
        """The same Raw Input checks with the listener started before Tauri builds its loop:
        recorded, not gated."""
        app = self.app
        rec: list = []
        st = app.launch(["--background", *shlex.split(self.a.before_build_args)])
        if not st["ok"]:
            self.notes["sink before build"] = {"error": st}
            return
        wait(lambda: tray_ok(app.state())[0], 10.0)
        self.sink_check("at start", rec)
        self.second_start(ROUTE)
        self.sink_check("after the window opens", rec)
        time.sleep(2)
        self.close_window()
        self.sink_check("after the window closes", rec)
        app.call("tray_churn", timeout=60)
        self.sink_check("after tray_churn", rec)
        self.notes["sink before build"] = rec
        app.call("quit")
        try:
            app.proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            pass
        app.kill_all()

    def baseline(self) -> None:
        app = self.app
        st = app.launch(shlex.split(self.a.baseline_args))
        if not st["ok"]:
            self.cost["plain tray"] = {"error": st}
            return
        time.sleep(3)
        self.notes["plain tray ui_state"] = app.state()
        self.cost["plain tray"] = measure(app, self.a.idle_seconds)
        app.call("quit")
        try:
            app.proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            pass
        app.kill_all()


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
                for v in ("Disabled", "DontShowUI", "LoggingDisabled", "Consent"):
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


def restart_report(a, psutil) -> dict:
    app = App(a, psutil)
    cases = [("abort", 65), ("access_violation", 65), ("main_thread_panic", 65), ("terminate", 65), ("abort", 10)]
    rep = {"wer": wer_state(psutil), "cases": []}
    print(json.dumps(rep["wer"]), flush=True)
    for kind, up in cases:
        app.kill_all()
        reports0 = wer_reports()
        st = app.launch(["--background", a.crash_hook_arg])
        case = {"kind": kind, "uptime_s": up}
        if not st["ok"]:
            case["error"] = st
            rep["cases"].append(case)
            continue
        t_start = time.time() - st["s"]
        time.sleep(max(0.0, up - (time.time() - t_start)))
        pid = app.proc.pid
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
        back = wait(lambda: restarted(app, pid), 90.0, 1.0)
        case["restarted"] = bool(back)
        case["restart_after_s"] = round(time.time() - t0, 1) if back else None
        if back:
            try:
                case["restart_cmdline"] = back[0].cmdline()
            except psutil.Error:
                pass
        case["new_wer_reports"] = sorted(Path(p).name for p in wer_reports() - reports0)
        print(json.dumps(case), flush=True)
        rep["cases"].append(case)
        for p in back or []:
            try:
                p.kill()
            except psutil.Error:
                pass
    app.kill_all()
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


def probe_markdown(p: Probe) -> str:
    lines = header("One process probe (astromech-wgtr)")
    lines += ["| Check | Verdict | Detail |", "|---|---|---|"]
    for cid in sorted(p.checks):
        c = p.checks[cid]
        d = json.dumps(c["detail"], default=str)
        lines.append(f"| {cid}. {c['name']} | {'PASS' if c['pass'] else '**FAIL**'} | `{d[:600]}` |")
    lines += ["", "## Raw Input with the listener started before Tauri's build (recorded, not gated)", ""]
    rec = p.notes.get("sink before build")
    if isinstance(rec, list):
        lines += ["| When | Keys before | after | Seen |", "|---|---|---|---|"]
        lines += [f"| {s['when']} | {s.get('before')} | {s.get('after')} | {'yes' if s['pass'] else 'no'} |" for s in rec]
    else:
        lines.append(f"Not run: {rec}")
    lines += ["", "## Cost (reported, not gated: decision astromech-01ft)", "",
              "Private working set (USS) and CPU of the app process plus the WebView2 processes that belong to it.", "",
              "| State | USS mean MB | USS max MB | CPU % of machine | CPU % of one core | Processes |",
              "|---|---|---|---|---|---|"]
    for k, m in p.cost.items():
        if "error" in m:
            lines.append(f"| {k} | error: {m['error']} | | | | |")
        else:
            lines.append(f"| {k} | {m['uss_mb_mean']} | {m['uss_mb_max']} | {m['cpu_machine_pct']} | "
                         f"{m['cpu_core_pct']} | {m['processes_max']} |")
    lines += ["", "## What a crashed window showed", ""]
    for c in p.notes.get("window crash", []):
        lines.append(f"- **{c['target']} killed** ({len(c.get('killed', []))} processes): window "
                     f"`{c.get('window_after_kill')}`, page beating 6 s later: {c.get('page_alive_after_kill')} "
                     f"(beats {c.get('page_beats_before')} → {c.get('page_beats_6s_after')}), WebView2 left: "
                     f"{[w['type'] for w in c.get('webview_after_kill', [])]}, ProcessFailed: "
                     f"{c.get('process_failed_log')}, screenshot: {c.get('screenshot')}; reopened: {c.get('reopened')} "
                     f"(beats after reopen {c.get('page_beats_after_reopen')}).")
    lines += ["", "The spike only logs WebView2's ProcessFailed event; it does nothing on it. A dead page that "
              "stays dead until the window is closed means the app needs its own ProcessFailed handler to "
              "recover in place."]
    return "\n".join(lines) + "\n"


def restart_markdown(rep: dict) -> str:
    lines = header("Restart registration (astromech-wgtr, reported, not gated)")
    lines += [f"WER on the runner: `{json.dumps(rep['wer'])}`", "",
              "Registered with `RegisterApplicationRestart(\"--background --restarted\", RESTART_NO_PATCH | "
              "RESTART_NO_REBOOT)`. A restart counts when a process with `--restarted` appears within 90 s.", "",
              "| Crash | Uptime | Exit code | Restarted | After | New WER reports |", "|---|---|---|---|---|---|"]
    for c in rep["cases"]:
        if "error" in c:
            lines.append(f"| {c['kind']} | {c['uptime_s']} s | error: {c['error']} | | | |")
            continue
        lines.append(f"| {c['kind']} | {c['uptime_s']} s | {c.get('exit_code')} | "
                     f"{'yes' if c['restarted'] else 'no'} | {c.get('restart_after_s')} | "
                     f"{', '.join(c.get('new_wer_reports', [])) or 'none'} |")
    return "\n".join(lines) + "\n"


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--app", type=Path, required=True, help="the app's exe")
    p.add_argument("--endpoint", type=Path, required=True, help="its core.json")
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--log", type=Path, default=None, help="its log (default: spike.log next to the endpoint)")
    p.add_argument("--user-data", type=Path, default=None,
                   help="its WebView2 user-data folder (default: EBWebView next to the endpoint)")
    p.add_argument("--title", default="astromech spike", help="the window's title")
    p.add_argument("--start-args", default="--sink-after-build", help="added to the main run's first start")
    p.add_argument("--before-build-args", default="--sink-before-build",
                   help="the recorded Raw Input run's extra start arguments; empty to skip it")
    p.add_argument("--baseline-args", default="--plain-tray", help="the cost baseline's start; empty to skip it")
    p.add_argument("--crash-hook-arg", default="--crash-hook")
    p.add_argument("--idle-seconds", type=float, default=30.0)
    p.add_argument("--restart", action="store_true", help="report restart registration instead")
    a = p.parse_args(argv)
    if os.name != "nt":
        print("one_process_probe.py runs on Windows only", file=sys.stderr)
        return 2
    import psutil

    a.out.mkdir(parents=True, exist_ok=True)
    if a.restart:
        rep = restart_report(a, psutil)
        (a.out / "restart.json").write_text(json.dumps(rep, indent=1, default=str), encoding="utf-8")
        md = restart_markdown(rep)
        (a.out / "restart.md").write_text(md, encoding="utf-8")
        print(md)
        return 0
    probe = Probe(a, psutil)
    try:
        probe.run()
        if a.before_build_args.strip():
            probe.before_build()
        if a.baseline_args.strip():
            probe.baseline()
    finally:
        probe.app.kill_all()
        rep = {"checks": probe.checks, "cost": probe.cost, "notes": probe.notes}
        (a.out / "report.json").write_text(json.dumps(rep, indent=1, default=str), encoding="utf-8")
        md = probe_markdown(probe)
        (a.out / "report.md").write_text(md, encoding="utf-8")
        print(md)
    expected = {"1", "2", "3", "4", "5", "6", "7"}
    passed = expected <= set(probe.checks) and all(c["pass"] for c in probe.checks.values())
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
