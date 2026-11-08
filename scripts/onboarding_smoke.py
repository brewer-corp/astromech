#!/usr/bin/env python3
"""Onboarding smoke test (Windows runner): a fresh per-user install, the onboarding wizard
driven in the real app window, then a verified "gear down" against the mock game, with the
time each part took.

    python scripts/onboarding_smoke.py --setup dist-release/astromech_x.y.z_x64-setup.exe \
        --mock target/release/mockgame.exe --out onboarding-report

What it does, in the order a new player would:
1. Elite has run once on this account: its Bindings folder (a custom preset with a keyboard
   key for the landing gear) and journal folder exist in their default places.
2. Silent install (`setup.exe /S`), then the app (`astromech-core.exe`, the core, tray and
   window in one process) is started from the install folder, with WebView2's remote debugging
   on (`--webview-debug-port`) so the page can be driven over the Chrome DevTools Protocol.
3. The wizard: Find Elite must show both folders found; push-to-talk and the kill switch are
   bound by pressing keys (SendInput; the window's press listener sees them) and saved;
   the mic and calibration steps are skipped (the runner has no mic); Finish.
4. The mock game starts as EliteDangerous64.exe (so the core's game watcher starts a session)
   and writes Status.json and a journal into the default journal folder.
5. "gear down" is typed into the voice tester page and run live (the runner can't speak, and
   the voice tester's live run goes through the same tool, policy and verification as a spoken
   command); the mock is brought to the front, and the page must show the game confirmed it.
6. Every top-level page (each module in app/src/modules) is opened, and none may show that a
   call didn't reach its command: "not allowed", "unknown command", an internal error, "the
   core is not running", or args the command or the window's `call` refused. Then the star map
   is opened (`galaxy_systems`), and the app's log must name the installed `starmap.db` as its
   preload.
7. The kill switch, with the window open and the session running: a tap of F8 (SendInput) must
   turn it on in `status`, and a 1.2 s hold, past `controls.rearm_hold_ms` (1000 ms by
   default), must re-arm it. Then the window is closed with WM_CLOSE, as a click on X would,
   and one more tap must turn the kill switch on with no window. Done once in the session from
   step 4, then again after the app is quit with `--quit` (config kept), the mock game started
   first and the app started again in the background, so the session's listener starts at app
   start, before any window (as after an update relaunch while Elite runs);
   a second start with `--route settings` opens the window before the presses. The app's one
   input source must register after Tauri builds its event loop, which registers Raw Input for
   its own window, and this second run is the one where the session's listener could start
   before that.
Then the app is quit and uninstalled. The report lists each step's time
and the total from the start of the install to the verified gear-down, against the 5 minute
onboarding budget (docs/V1-DESIGN.md: install to working gear-down in under 5 minutes; the
timed human run is separate).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import socket
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MODULES = ROOT / "app" / "src" / "modules"
BUDGET_S = 5 * 60
GEAR_DOWN = "gear down verified"
# What a page shows when a call didn't reach its command, lower case: the registry's refusals and
# a caught panic (crates/astromech-api/src/lib.rs), the control client with no core, and Tauri
# refusing the proxy's own arguments.
BROKEN = ("not allowed", "unknown command", "internal error", "the core is not running", "bad args for",
          "invalid args")
CDP_PORT = 9223
ELITE_TITLE = "Elite - Dangerous (CLIENT)"
# Set-1 scan codes: the binds press Key_L for the gear; the wizard binds F7 and F8.
SCAN_L, SCAN_F7, SCAN_F8 = 0x26, 0x41, 0x42
# Longer than controls.rearm_hold_ms's default (1000 ms, crates/astromech-core/src/config.rs).
REARM_HOLD_S = 1.2
GEAR = 1 << 2

BINDS = """<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="Custom" MajorVersion="4" MinorVersion="2">
 <LandingGearToggle><Primary Device="Keyboard" Key="Key_L" /><Secondary Device="{NoDevice}" Key="" /></LandingGearToggle>
</Root>
"""

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

    user32 = ctypes.windll.user32
    user32.SendInput.argtypes = [wintypes.UINT, ctypes.c_void_p, ctypes.c_int]
    user32.FindWindowW.argtypes = [wintypes.LPCWSTR, wintypes.LPCWSTR]
    user32.FindWindowW.restype = wintypes.HWND
    user32.SetForegroundWindow.argtypes = [wintypes.HWND]
    user32.ShowWindow.argtypes = [wintypes.HWND, ctypes.c_int]
    user32.GetForegroundWindow.restype = wintypes.HWND
    user32.PostMessageW.argtypes = [wintypes.HWND, wintypes.UINT, wintypes.WPARAM, wintypes.LPARAM]
    user32.GetWindowThreadProcessId.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.DWORD)]
    user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
    user32.GetWindowTextLengthW.argtypes = [wintypes.HWND]
    _ENUM = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)

APP = "astromech-core.exe"
# The window's title (app/src-tauri/src/host.rs).
TITLE = "astromech"


def send(vk: int, scan: int, up: bool) -> None:
    i = _INPUT(type=1)
    flags = (0x8 if scan else 0) | (0x2 if up else 0)
    i.u.ki = _KEYBDINPUT(vk, scan, flags, 0, 0)
    if user32.SendInput(1, ctypes.byref(i), ctypes.sizeof(_INPUT)) != 1:
        raise OSError("SendInput failed")


def tap(scan: int) -> None:
    send(0, scan, False)
    time.sleep(0.1)
    send(0, scan, True)


def bring_to_front(title: str, timeout: float = 4.0) -> bool:
    """Make the window with this title the foreground window. Windows only lets a process do
    that in some cases; tapping Alt first is the usual way to be allowed."""
    end = time.time() + timeout
    while time.time() < end:
        h = user32.FindWindowW(None, title)
        if h:
            send(0x12, 0, False)
            send(0x12, 0, True)
            user32.ShowWindow(h, 9)
            user32.SetForegroundWindow(h)
            if user32.GetForegroundWindow() == h:
                return True
        time.sleep(0.1)
    return False


def webview_diagnostics() -> str:
    """The WebView2 processes' command lines and the listening ports, for a failed connect."""
    ps = ("Get-CimInstance Win32_Process -Filter \"name='msedgewebview2.exe'\" | "
          "ForEach-Object { $_.CommandLine.Substring(0, [Math]::Min(400, $_.CommandLine.Length)) }")
    procs = subprocess.run(["powershell", "-NoProfile", "-Command", ps], capture_output=True, text=True).stdout
    ports = subprocess.run(["netstat", "-ano", "-p", "TCP"], capture_output=True, text=True).stdout
    listening = [l.split()[1] for l in ports.splitlines() if "LISTENING" in l]
    return f"webview2 processes: {procs.strip()[:2000]!r}; listening: {listening}"


class Page:
    """The app window's page over the Chrome DevTools Protocol (WebView2 remote debugging)."""

    def __init__(self, port: int, timeout: float):
        import websocket  # websocket-client

        end = time.time() + timeout
        target = None
        while time.time() < end and target is None:
            try:
                with urllib.request.urlopen(f"http://127.0.0.1:{port}/json", timeout=2) as r:
                    target = next((t for t in json.load(r) if t.get("type") == "page"), None)
            except (OSError, ValueError):
                pass
            if target is None:
                time.sleep(0.2)
        if target is None:
            raise RuntimeError(f"no WebView2 page on the debugging port {port}; {webview_diagnostics()}")
        self.ws = websocket.create_connection(target["webSocketDebuggerUrl"], timeout=30, suppress_origin=True)
        self.n = 0

    def eval(self, expr: str):
        self.n += 1
        self.ws.send(json.dumps({"id": self.n, "method": "Runtime.evaluate",
                                 "params": {"expression": expr, "awaitPromise": True, "returnByValue": True}}))
        while True:
            m = json.loads(self.ws.recv())
            if m.get("id") == self.n:
                r = m.get("result", {})
                if "exceptionDetails" in r:
                    raise RuntimeError(f"page error: {r['exceptionDetails']}")
                return r.get("result", {}).get("value")

    def wait(self, expr: str, what: str, timeout: float = 20.0):
        end = time.time() + timeout
        while time.time() < end:
            v = self.eval(expr)
            if v:
                return v
            time.sleep(0.1)
        raise RuntimeError(f"timed out waiting for {what}; page text: {self.text()[:600]!r}")

    def text(self) -> str:
        return self.eval("document.body ? document.body.innerText : ''") or ""

    def has(self, s: str) -> str:
        return f"document.body && document.body.innerText.includes({json.dumps(s)})"

    def click(self, label: str, what: str, timeout: float = 10.0) -> None:
        """Click the enabled button whose text is exactly `label`."""
        js = ("(() => { const b = [...document.querySelectorAll('button')].find(b => b.textContent.trim() === "
              f"{json.dumps(label)} && !b.disabled); if (!b) return false; b.click(); return true; }})()")
        self.wait(js, what, timeout)


def endpoint() -> Path:
    return Path(os.environ["LOCALAPPDATA"]) / "astromech" / "data" / "core.json"


def core_call(cmd: str, **extra) -> dict:
    """One control-API request: the command's value. Skips progress lines; raises ValueError
    with the core's error if the reply isn't ok."""
    ep = json.loads(endpoint().read_text())
    with socket.create_connection(("127.0.0.1", ep["port"]), timeout=5) as s:
        s.sendall((json.dumps({"token": ep["token"], "cmd": cmd, **extra}) + "\n").encode())
        for line in s.makefile():
            reply = json.loads(line)
            if "progress" in reply:
                continue
            if reply.get("ok") is not True:
                raise ValueError(f"{cmd}: {reply.get('error')}")
            return reply["value"]
        raise ValueError(f"{cmd}: the core closed the connection without answering")


def wait_core(pred, what: str, timeout: float) -> dict:
    end = time.time() + timeout
    last: dict = {}
    while time.time() < end:
        try:
            last = core_call("status")
            if pred(last):
                return last
        except (OSError, ValueError, KeyError):
            pass
        time.sleep(0.25)
    raise RuntimeError(f"timed out waiting for {what}; last status: {json.dumps(last)[:600]}")


def install_dir() -> Path:
    la = Path(os.environ["LOCALAPPDATA"])
    for d in (la / "Programs" / "astromech", la / "astromech"):
        if (d / APP).is_file():
            return d
    raise RuntimeError(f"{APP} not installed")


def app_window(pid: int) -> int | None:
    """The app's window, found by its title among `pid`'s top-level windows."""
    found = []

    def cb(h, _):
        p = wintypes.DWORD()
        user32.GetWindowThreadProcessId(h, ctypes.byref(p))
        if p.value == pid:
            n = user32.GetWindowTextLengthW(h)
            buf = ctypes.create_unicode_buffer(n + 1)
            user32.GetWindowTextW(h, buf, n + 1)
            if buf.value == TITLE:
                found.append(h)
        return True

    user32.EnumWindows(_ENUM(cb), 0)
    return found[0] if found else None


def window_state() -> str | None:
    """`ui_state`'s window: "open" or "closed"."""
    try:
        return core_call("ui_state")["window"]
    except (OSError, ValueError, KeyError):
        return None


def close_window(run: Run, when: str) -> None:
    """WM_CLOSE to the window, as a click on X: it is destroyed and the app stays."""
    pid = json.loads(endpoint().read_text())["pid"]
    hwnd = app_window(pid)
    if not hwnd:
        raise RuntimeError(f"no window titled {TITLE!r} in the app's process ({when})")
    user32.PostMessageW(hwnd, 0x0010, 0, 0)
    end = time.time() + 10
    while window_state() != "closed":
        if time.time() > end:
            raise RuntimeError(f"the window didn't close within 10 s of WM_CLOSE ({when})")
        time.sleep(0.25)
    run.step(f"window closed with WM_CLOSE, the app still running ({when})")


class Run:
    def __init__(self):
        self.t0 = time.time()
        self.last = self.t0
        self.steps: list[dict] = []
        self.notes: list[str] = []

    def step(self, name: str) -> None:
        now = time.time()
        self.steps.append({"step": name, "seconds": round(now - self.last, 2), "at": round(now - self.t0, 2)})
        print(f"[{now - self.t0:7.2f} s] {name} ({now - self.last:.2f} s)", flush=True)
        self.last = now


def kill(name: str) -> None:
    subprocess.run(["taskkill", "/F", "/IM", name], capture_output=True)


def smoke(a, run: Run) -> None:
    la, home = Path(os.environ["LOCALAPPDATA"]), Path(os.environ["USERPROFILE"])
    # A fresh install: no astromech data from an earlier step on this runner.
    for p in ("astromech-shell.exe", APP):  # astromech-shell.exe: an older two-process install
        kill(p)
    shutil.rmtree(la / "astromech" / "data", ignore_errors=True)
    # Elite has run once on this account.
    bindings = la / "Frontier Developments" / "Elite Dangerous" / "Options" / "Bindings"
    journal = home / "Saved Games" / "Frontier Developments" / "Elite Dangerous"
    bindings.mkdir(parents=True, exist_ok=True)
    journal.mkdir(parents=True, exist_ok=True)
    (bindings / "Custom.4.2.binds").write_text(BINDS, encoding="utf-8")
    (bindings / "StartPreset.4.start").write_text("Custom\nCustom\nCustom\nCustom\n", encoding="utf-8")
    run.t0 = run.last = time.time()

    subprocess.run([str(a.setup), "/S"], check=True)
    inst = install_dir()
    run.step(f"silent install ({inst})")

    procs: list[subprocess.Popen] = []
    try:
        drive(a, run, inst, journal, procs)
    finally:
        cleanup(inst, procs)


def cleanup(inst: Path, procs: list) -> None:
    """Close what the run started and uninstall; a failure here doesn't hide the run's own error."""
    for p in procs:
        p.kill()
    for cmd, timeout in (([str(inst / APP), "--quit"], 30), ([str(inst / "uninstall.exe"), "/S"], 120)):
        try:
            subprocess.run(cmd, timeout=timeout)
        except (OSError, subprocess.SubprocessError) as e:
            print(f"cleanup: {cmd[0]}: {e}", flush=True)


def drive(a, run: Run, inst: Path, journal: Path, procs: list) -> None:
    procs.append(subprocess.Popen([str(inst / APP), "--webview-debug-port", str(CDP_PORT)]))
    page = Page(CDP_PORT, 60)
    page.wait(page.has("Find Elite"), "the onboarding wizard", 60)
    run.step("app window open on the wizard")

    # 1. Find Elite.
    page.wait("[...document.querySelectorAll('.setup-step dd')].filter(d => d.innerText.includes('Found') && "
              "!d.innerText.includes('Not found')).length >= 2", "both of Elite's folders found")
    page.click("Next", "Next")
    # 2. Controllers.
    page.wait(page.has("Controllers"), "the controllers step")
    page.click("Next", "Next")
    # 3. Push-to-talk and kill switch, by pressing the keys.
    page.wait(page.has("Push-to-talk"), "the buttons step")
    for i, (scan, which) in enumerate(((SCAN_F7, "ptt"), (SCAN_F8, "kill_switch"))):
        page.wait(f"(() => {{ const b = document.querySelectorAll('.setup-step dd button')[{i}]; "
                  "if (!b) return false; b.click(); return true; })()", f"the {which} Set button")
        page.wait("!!document.querySelector('.press')", "the press prompt")
        end = time.time() + 10
        while time.time() < end and not page.eval("!!document.querySelector('.press.got')"):
            tap(scan)
            time.sleep(0.5)
        page.click("Save", f"Save ({which})")
        wait_core(lambda s, w=which: bool(s.get(w)), f"{which} saved in the config", 10)
    run.step("push-to-talk and kill switch bound by pressing keys")
    page.click("Next", "Next")
    # 4-5. Microphone and calibration: no mic on the runner.
    page.wait(page.has("Microphone"), "the mic step")
    page.click("Next", "Next")
    page.wait(page.has("Calibration"), "the calibration step")
    page.click("Next", "Next")
    # 6. Health check, then Finish.
    page.wait(page.has("Health check"), "the health step")
    page.click("Finish", "Finish")
    st = wait_core(lambda s: s.get("onboarded") is True, "onboarding marked done", 10)
    run.step("wizard finished")
    run.notes.append(f"after onboarding: ptt {st.get('ptt')}, kill switch {st.get('kill_switch')}, "
                     f"still to set up: {st.get('setup_needed')}")

    # The game starts.
    game = Path(tempfile.mkdtemp(prefix="mock-elite-")) / "EliteDangerous64.exe"
    shutil.copy(a.mock, game)
    mock = start_mock(a, game, journal)
    procs.append(mock)
    wait_core(lambda s: s.get("active") is True, "the core's session (game detected)", 30)
    run.step("game detected, session running")

    page.eval("location.hash = '#/voicetester'")
    page.wait("!!document.querySelector('input[aria-label=\"Phrase\"]')", "the voice tester")
    page.eval("(() => { const i = document.querySelector('input[aria-label=\"Phrase\"]'); i.value = 'gear down'; "
              "i.dispatchEvent(new Event('input', { bubbles: true })); i.form.requestSubmit(); })()")
    page.click("Run live", "Run live (needs the session)", 15)
    t_live = time.time()
    if not bring_to_front(ELITE_TITLE):
        raise RuntimeError("couldn't bring the mock game to the front")
    note = page.wait("(() => { const n = document.querySelector('.note'); return n && n.innerText; })()",
                     "the live run's result", 15)
    run.notes.append(f"voice tester: {note}")
    if "confirmed it in" not in note:
        raise RuntimeError(f"gear down wasn't verified: {note}")
    run.step(f"{GEAR_DOWN} ({time.time() - t_live:.2f} s from Run live)")
    keys = (journal / "keys.log").read_text(encoding="utf-8")
    flags = json.loads((journal / "Status.json").read_text(encoding="utf-8"))["Flags"]
    if "down scan=26" not in keys or not flags & GEAR:
        raise RuntimeError("the mock didn't get the gear key, or its gear flag isn't set")

    visit_pages(page, run)
    star_map_preload(inst, run)
    kill_switch(run, "session started after the app")
    restart_with_game_running(a, run, inst, game, journal, procs, app=procs[0], mock=mock)


def start_mock(a, game: Path, journal: Path) -> subprocess.Popen:
    """The mock game as EliteDangerous64.exe: the gear on Key_L, writing into `journal`."""
    return subprocess.Popen([str(game), str(journal), f"{SCAN_L:x}", "0", str(GEAR), "0-1000", "600",
                             str(a.mock_drop)])


def kill_switch(run: Run, when: str) -> None:
    """With the window open, tap the bound kill switch (F8): on. Hold it past the re-arm hold:
    re-armed. Then close the window and tap again: on. Read from the core's `status`, so it's
    the session's own listener that saw the presses."""
    if window_state() != "open":
        raise RuntimeError(f"the window isn't open for the kill switch presses ({when})")
    wait_core(lambda s: s.get("killed") is False, f"the kill switch armed ({when})", 10)
    tap(SCAN_F8)
    wait_core(lambda s: s.get("killed") is True, f"the kill switch on after a tap of F8 ({when})", 10)
    send(0, SCAN_F8, False)
    time.sleep(REARM_HOLD_S)
    send(0, SCAN_F8, True)
    wait_core(lambda s: s.get("killed") is False, f"the kill switch re-armed by a {REARM_HOLD_S} s hold ({when})", 10)
    run.step(f"kill switch: a tap of F8 killed, a {REARM_HOLD_S} s hold re-armed, window open ({when})")
    close_window(run, when)
    tap(SCAN_F8)
    wait_core(lambda s: s.get("killed") is True, f"the kill switch on after a tap of F8, window closed ({when})", 10)
    run.step(f"kill switch: a tap of F8 killed with the window closed ({when})")


def restart_with_game_running(a, run: Run, inst: Path, game: Path, journal: Path, procs: list,
                              app: subprocess.Popen, mock: subprocess.Popen) -> None:
    """Quit (config kept), start the mock game, then the app: its session, and so its input
    listener, starts at app start, before any window. Open the window by a second start, then
    check the kill switch again."""
    exe = str(inst / APP)
    subprocess.run([exe, "--quit"], timeout=30)
    end = time.time() + 30
    while endpoint().exists():  # a clean quit removes it
        if time.time() > end:
            raise RuntimeError(f"{APP} --quit didn't stop the app")
        time.sleep(0.25)
    try:
        code = app.wait(10)
    except subprocess.TimeoutExpired:
        raise RuntimeError(f"the app's process was still running 10 s after its endpoint went") from None
    run.notes.append(f"--quit: the app exited with {code}")
    mock.kill()
    mock.wait(10)
    run.step("app quit, mock game closed")

    procs.append(start_mock(a, game, journal))
    time.sleep(1)
    procs.append(subprocess.Popen([exe, "--background"]))
    wait_core(lambda s: s.get("active") is True, "the session at app start (game already running)", 30)
    if window_state() != "closed":
        raise RuntimeError("--background opened a window")
    run.step("mock game started, then the app in the background: session running, no window")
    second = subprocess.run([exe, "--route", "settings"], timeout=30)
    if second.returncode != 0:
        raise RuntimeError(f"the second start exited with {second.returncode}")
    end = time.time() + 30
    while window_state() != "open":
        if time.time() > end:
            raise RuntimeError("a second start with --route settings opened no app window")
        time.sleep(0.25)
    time.sleep(3)  # the window's page loading, as a player would open it
    run.step("window opened by a second start (--route settings)")
    kill_switch(run, "game running before the app")


def module_ids() -> list[str]:
    """The window's top-level pages: each module's id, its route (`#/<id>`)."""
    ids = []
    for m in sorted(MODULES.glob("*/manifest.ts")):
        found = re.search(r'\bid:\s*"([a-z0-9-]+)"', m.read_text(encoding="utf-8"))
        if not found:
            raise RuntimeError(f"no id in {m}")
        ids.append(found.group(1))
    if not ids:
        raise RuntimeError(f"no modules in {MODULES}")
    return ids


def settle(page: Page, quiet: float = 1.5, timeout: float = 15.0) -> str:
    """The page's text once it has stopped changing for `quiet` seconds (its calls answered)."""
    end = time.time() + timeout
    last, since = None, time.time()
    while time.time() < end:
        t = page.text()
        if t != last:
            last, since = t, time.time()
        elif time.time() - since >= quiet:
            break
        time.sleep(0.25)
    return last or ""


def broken_lines(text: str) -> list[str]:
    return [l.strip() for l in text.splitlines() if any(b in l.lower() for b in BROKEN)]


def visit_pages(page: Page, run: Run) -> None:
    """Open every top-level page; fail on any that shows a call that didn't reach its command."""
    ids = module_ids()
    bad = {}
    for m in ids:
        page.eval(f"location.hash = {json.dumps('#/' + m)}")
        text = settle(page)
        if lines := broken_lines(text):
            bad[m] = lines[:3]
        if m == "diagnostics":
            version = re.search(r"Version\n(.*)", text)
            run.notes.append(f"Diagnostics version: {version.group(1) if version else 'not shown'}")
    if bad:
        raise RuntimeError(f"pages showing a call that didn't reach its command: {json.dumps(bad)}")
    run.step(f"{len(ids)} pages opened, none refused a call")


def redacted(p: Path) -> str:
    """`p` as the core's log writes it: the log redacts %LOCALAPPDATA% (astromech-keybinds Redactor)."""
    la = os.environ["LOCALAPPDATA"]
    s = str(p)
    return "%LOCALAPPDATA%" + s[len(la):] if s.lower().startswith(la.lower()) else s


def star_map_preload(inst: Path, run: Run) -> None:
    """Open the star map (it opens on first use) and find the core's line naming its preload."""
    try:
        core_call("galaxy_systems", args={"prefix": "Sol"})
    # Spansh's suggestions may fail or outlast core_call's 5 s socket timeout (an OSError); the star
    # map opened before they were asked.
    except (ValueError, OSError) as e:
        run.notes.append(f"galaxy_systems: {e}")
    log = Path(os.environ["LOCALAPPDATA"]) / "astromech" / "data" / "logs" / "astromech-core.log"
    want = f"preload {redacted(inst / 'starmap.db')}".lower()
    end, lines = time.time() + 10, []
    while time.time() < end:
        lines = [l for l in log.read_text(encoding="utf-8", errors="replace").splitlines() if "star map:" in l]
        if any(want in l.lower() for l in lines):
            run.notes.append(f"core log: {next(l for l in lines if want in l.lower()).strip()}")
            run.step("star map opened with the installed preload")
            return
        time.sleep(0.25)
    raise RuntimeError(f"the core's log has no star map line with {want!r}; its star map lines: {lines}")

def report_md(r: dict) -> str:
    lines = ["# Onboarding smoke test: fresh install to a verified gear-down (mock game)", "",
             f"Result: **{'PASS' if r['pass'] else 'FAIL'}**. Install to verified gear-down: "
             f"{r['total_s'] if r['total_s'] is not None else 'not reached'} s (budget {BUDGET_S} s).", "",
             "| Step | Seconds | At (s) |", "|---|---|---|"]
    lines += [f"| {s['step']} | {s['seconds']} | {s['at']} |" for s in r["steps"]]
    if r["error"]:
        lines += ["", f"Error: {r['error']}"]
    lines += [""] + [f"- {n}" for n in r["notes"]]
    lines += ["", "Machine time: no reading, thinking or speaking. The timed human run is astromech-we0f."]
    return "\n".join(lines) + "\n"


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--setup", type=Path, required=True, help="the NSIS installer")
    p.add_argument("--mock", type=Path, required=True, help="mockgame.exe")
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--mock-drop", type=int, default=0,
                   help="fault injection: the mock drops its first N gear presses (the run must then fail)")
    a = p.parse_args(argv)
    if os.name != "nt":
        print("needs Windows")
        return 2
    a.out.mkdir(parents=True, exist_ok=True)
    run = Run()
    error = None
    try:
        smoke(a, run)
    except Exception as e:  # report what got done, then fail
        error = f"{type(e).__name__}: {e}"
    # The onboarding time is install to the verified gear-down; the page tour after it isn't in it.
    total = next((s["at"] for s in run.steps if s["step"].startswith(GEAR_DOWN)), None)
    reached = total is not None
    r = {"pass": reached and error is None and total <= BUDGET_S, "total_s": total, "budget_s": BUDGET_S,
         "steps": run.steps, "notes": run.notes, "error": error}
    (a.out / "report.json").write_text(json.dumps(r, indent=1), encoding="utf-8")
    md = report_md(r)
    (a.out / "report.md").write_text(md, encoding="utf-8")
    print(md)
    return 0 if r["pass"] else 1


if __name__ == "__main__":
    sys.exit(main())
