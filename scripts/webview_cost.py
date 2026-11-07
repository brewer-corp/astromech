#!/usr/bin/env python3
"""App window cost and the one-process checks: the one app windowless, with its window open,
and closed again (docs/ARCHITECTURE.md §18.1; success criterion one-process of mission
astromech-kjfs).

    python scripts/webview_cost.py --app target/release/astromech-core.exe --out webview-report
    python scripts/webview_cost.py --app "<install>/astromech-core.exe" --out <dir> --checks-only

Runs the one app (`astromech-core`: the core, the tray and the window in one process; WebView2 on
Windows, WebKitGTK on Linux) in a scratch profile, as a player uses it:

1. Cold start, new profile: started with no arguments, which opens the window and makes the
   WebView's profile; then quit.
2. Windowless: started with `--background` (Start with Windows), left to settle, and measured
   idle. No WebView process may belong to it, and it is the profile's one app process.
3. Open: a second start with `--route keybinds` opens the window (it hands over and exits 0).
   Measured idle on that page, then over page changes: a second start with `--route <page>`
   every 2 s, each of which must reach the window.
4. Closed again, 5 cycles: WM_CLOSE to the window, as a click on X; the time until no WebView2
   process belonging to the app is left (at most 15 s), and the app's private working set 30 s
   after the close, per cycle, so a leak would show. Each cycle after the first reopens the
   window by a second start.
5. Window crash: the renderer killed, then the browser process. The app and its control API
   must live, the window must recover in place (a reload; a rebuild), and closing the crashed
   window must free WebView2 as any close does (finding astromech-k6f1).
6. Quit with the window open: exit code 0 and no WebView2 left.
7. Cold start with the app not running, on the profile it made.

Cold start runs from a start to the window's first `window: page loaded` log line: the median of
the window opens in the app running windowless (steps 3 and 4: five on Windows), and step 7.

A WebView2 process belongs to the app when it descends from the app, or its command line has
`--webview-exe-name=<the app's exe name>`, or it names the app's `EBWebView` folder, so a
renderer orphaned by a dead parent still counts (the rule of spike astromech-wgtr). So run it
with no other astromech-core on the machine. Linux has descent only and no WM_CLOSE: steps 4
and 5 are Windows only, and the report says they didn't run.

Gates (exit status 1 if one fails): the app windowless with no WebView2; the window opened by a
second start; no WebView2 within 15 s of each close; the app alive after each close and crash;
the crashed window recovered and closed; quit; the page changes reached the window; cold start
under 2 s; idle CPU at most 1% of the machine, windowless and open. The tray is read after each
step: its icon in the notification area passes, and an icon built with no rectangle there (a
session with no notification area) is DEGRADED, which is shown but doesn't fail the run. Memory
is reported, with no pass mark (decision astromech-01ft).

`--checks-only` (Release's installer job, on the installed app) skips the page changes, the
30 s waits after each close and both cold starts, and doesn't judge CPU or cold start.
Frame-time impact next to Elite needs the game (astromech-7ck3). Needs psutil.
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import shutil
import socket
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

IDLE_CPU_MAX = 1.0  # % of the machine
COLD_START_MAX_S = 2.0
CLOSE_MAX_S = 15.0  # from WM_CLOSE to no WebView2 process of the app's
AFTER_CLOSE_S = 30.0  # the app's memory is read this long after each close
CYCLES = 5
LIST_PAGE = "keybinds"
PAGES = ["keybinds", "materials", "engineering", "trade", "logbook", "exploration", "outfitting", "settings"]
# The window's title (app/src-tauri/src/host.rs), found as a click on X would close it.
TITLE = "astromech"
# The app's Tauri identifier: WebView2 keeps its profile in <local app data>/<identifier>/EBWebView.
IDENTIFIER = "io.github.spadaval.astromech"

# The spike's numbers on windows-latest (astromech-wgtr, evidence astromech-t55z and astromech-mb4f),
# MB of private working set: the baseline the memory rows are shown against, not a bound.
SPIKE_MB = {"windowless": "3.3 to 3.5", "open": "124.1 mean, 126.3 max", "closed": "9.6, 9.7, 8.8, 7.9, 7.7"}

PASS, FAIL, DEGRADED, REPORTED, NOT_RUN = "pass", "fail", "degraded", "reported", "not run"


# --- Judging ------------------------------------------------------------------------------------


def close_ok(c: dict | None) -> bool:
    """A close passes when the window went and no WebView2 process of the app's was left within
    CLOSE_MAX_S of WM_CLOSE."""
    return bool(c) and c.get("window_closed_s") is not None and c.get("webview_gone_s") is not None \
        and c["webview_gone_s"] <= CLOSE_MAX_S


def tray_verdict(readings: list[str]) -> str:
    """`rect` (the icon has a rectangle in the notification area) passes; `fallback` (the icon was
    built, but there's no rectangle) is a degradation, not a pass (finding astromech-k6f1);
    anything else, or no reading at all, fails."""
    if not readings or any(t not in ("rect", "fallback") for t in readings):
        return FAIL
    return PASS if all(t == "rect" for t in readings) else DEGRADED


def judge(r: dict) -> list[dict]:
    """The gates and the reported rows for the results `r` of `measure`. A gate whose step was
    skipped on purpose (`r["skipped"]`) is `not run`; one whose data is missing fails."""
    skipped = r.get("skipped", {})
    rows: list[dict] = []

    def row(rid, check, value, verdict, skip=None):
        if skip in skipped:
            verdict, value = NOT_RUN, f"{skipped[skip]}" + (f"; {value}" if value else "")
        rows.append({"id": rid, "check": check, "value": value, "verdict": verdict})

    def gate(rid, check, value, ok, skip=None):
        row(rid, check, value, PASS if ok else FAIL, skip)

    wl = r.get("windowless") or {}
    gate("windowless", "the app with `--background`: one app process, no WebView2 process",
         f"{wl.get('app_processes')} app process(es); WebView2 seen: {wl.get('webview_pids', 'not measured')}",
         wl.get("alive") is True and wl.get("app_processes") == 1 and wl.get("webview_pids") == [])

    op = r.get("open") or {}
    gate("open", "a second start with `--route` opens the window",
         f"exit code {op.get('exit_code')}, page loaded after {op.get('page_load_s')} s, "
         f"{len(op.get('webview') or [])} WebView processes",
         op.get("exit_code") == 0 and op.get("page_load_s") is not None and bool(op.get("webview")))

    use = r.get("in_use") or {}
    gate("page_changes", "every page change reached the running window",
         f"{use.get('switches', 0)} asked, {use.get('pages_received', 0)} received, "
         f"{use.get('extra_windows', 0)} extra windows, exit codes {use.get('switch_exit_codes')}",
         use.get("window_alive") is True and use.get("switches", 0) > 0
         and use.get("pages_received") == use.get("switches") and use.get("extra_windows") == 0
         and use.get("switch_exit_codes") == [0], "page_changes")

    closes = r.get("closes") or []
    gate("close", f"no WebView2 process left {CLOSE_MAX_S:g} s after each close ({CYCLES} cycles)",
         "; ".join(f"{c.get('cycle')}: gone after {c.get('webview_gone_s')} s"
                   + (f", left {[w.get('pid') for w in c.get('webview_left')]}" if c.get("webview_left") else "")
                   + (f", {c['error']}" if c.get("error") else "") for c in closes) or "no closes",
         len(closes) == CYCLES and all(close_ok(c) for c in closes), "closes")

    crashes = r.get("crashes") or []
    events = [(f"close {c.get('cycle')}", c) for c in closes] + [(f"{c.get('target')} crash", c) for c in crashes]
    gate("app_alive", "the app and its control API alive after each close and crash",
         ", ".join(f"{name}: {'alive' if e.get('app_alive') and e.get('control') else 'gone'}"
                   for name, e in events) or "nothing ran",
         len(closes) == CYCLES and len(crashes) == 2
         and all(e.get("app_alive") is True and e.get("control") is True for _, e in events), "closes")

    gate("window_crash", "a crashed window recovers in place, and its close frees WebView2",
         "; ".join(f"{c.get('target')}: killed {len(c.get('killed') or [])}, recovered {c.get('recovered')}, "
                   f"WebView2 gone {(c.get('close') or {}).get('webview_gone_s')} s after the close"
                   + (f", {c['error']}" if c.get("error") else "") for c in crashes) or "no crashes",
         [c.get("target") for c in crashes] == ["renderer", "browser"]
         and all(c.get("killed") and c.get("recovered") is True and close_ok(c.get("close")) for c in crashes),
         "crashes")

    q = r.get("quit") or {}
    gate("quit", "quit with the window open: exit code 0, no WebView2 left",
         f"exit code {q.get('exit_code')}, WebView2 left {q.get('webview_left')}",
         q.get("exit_code") == 0 and q.get("webview_left") == [])

    cs = r.get("cold_start") or {}
    opens = cs.get("windowless_s") or []
    med = statistics.median(opens) if opens else None
    nr = cs.get("not_running_s")
    gate("cold_start", f"cold start under {COLD_START_MAX_S:g} s",
         f"median {None if med is None else round(med, 2)} s over {len(opens)} window opens in the app running "
         f"windowless; {nr} s with the app not running (first start, new profile: {cs.get('first_s')} s)",
         med is not None and med < COLD_START_MAX_S and nr is not None and nr < COLD_START_MAX_S, "cold_start")

    wcost, ocost = wl.get("cost") or {}, op.get("cost") or {}
    gate("idle_cpu_windowless", f"idle CPU windowless at most {IDLE_CPU_MAX:g}% of the machine",
         f"{wcost.get('cpu_machine_pct')}% ({wcost.get('cpu_core_pct')}% of one core)",
         wcost.get("cpu_machine_pct") is not None and wcost["cpu_machine_pct"] <= IDLE_CPU_MAX, "cpu")
    gate("idle_cpu_open", f"idle CPU with the window open at most {IDLE_CPU_MAX:g}% of the machine",
         f"{ocost.get('cpu_machine_pct')}% ({ocost.get('cpu_core_pct')}% of one core); in use "
         f"{(use.get('cost') or {}).get('cpu_machine_pct')}%",
         ocost.get("cpu_machine_pct") is not None and ocost["cpu_machine_pct"] <= IDLE_CPU_MAX
         and op.get("webview_seen_idle") is True, "cpu")

    trays = [t for t in [wl.get("tray")] + [e.get("tray") for _, e in events] if t is not None]
    row("tray", "the tray icon in the notification area after each step",
        ", ".join(trays) or "not read", tray_verdict(trays))

    # Memory: reported, not bounded (decision astromech-01ft).
    row("memory_windowless", "memory windowless (private working set)",
        f"{wcost.get('uss_mb_mean')} MB mean, {wcost.get('uss_mb_max')} MB max, {wcost.get('processes_max')} process(es) "
        f"(spike: {SPIKE_MB['windowless']} MB)",
        REPORTED)
    row("memory_open", f"memory with the window open, idle on `{LIST_PAGE}`",
        f"{ocost.get('uss_mb_mean')} MB mean, {ocost.get('uss_mb_max')} MB max, {ocost.get('processes_max')} processes; "
        f"in use peak {(use.get('cost') or {}).get('uss_mb_max')} MB (spike: {SPIKE_MB['open']} MB)", REPORTED)
    row("memory_closed", f"memory {AFTER_CLOSE_S:g} s after each close",
        (", ".join(f"{c.get('cycle')}: {c.get('uss_after_mb')} MB" for c in closes) or "no closes")
        + f" (spike: {SPIKE_MB['closed']} MB)", REPORTED, "closes")
    return rows


def failed(rows: list[dict]) -> bool:
    return any(c["verdict"] == FAIL for c in rows)


VERDICT_TEXT = {PASS: "PASS", FAIL: "**FAIL**", DEGRADED: "**DEGRADED**", REPORTED: "reported", NOT_RUN: "not run"}


def markdown(rep: dict) -> str:
    r = rep["results"]
    lines = ["# App window cost and the one process", "",
             f"{rep['webview']} on {rep['host']}, commit `{rep['commit']}`, {rep['started']}"
             + (", `--checks-only`" if r.get("checks_only") else "") + ".", "",
             "| Check | Measured | Verdict |", "|---|---|---|"]
    lines += [f"| {c['check']} | {c['value']} | {VERDICT_TEXT[c['verdict']]} |" for c in rep["checks"]]
    if r.get("error"):
        lines += ["", f"**The run stopped early:** `{r['error']}`"]
    closes = r.get("closes")
    if closes:
        lines += ["", f"## Open and closed again ({CYCLES} cycles)", "",
                  "| Cycle | Reopen: page load s | Window closed s | WebView2 gone s | App alive | Tray | "
                  f"Private working set {AFTER_CLOSE_S:g} s after MB | Processes |", "|---|---|---|---|---|---|---|---|"]
        for c in closes:
            lines.append(f"| {c.get('cycle')} | {(c.get('reopen') or {}).get('page_load_s', '(the open)')} | "
                         f"{c.get('window_closed_s')} | {c.get('webview_gone_s')} | {c.get('app_alive')} | "
                         f"{c.get('tray')} | {c.get('uss_after_mb')} | {c.get('processes_after')} |")
    crashes = r.get("crashes")
    if crashes:
        lines += ["", "## Window crash", ""]
        for c in crashes:
            lines.append(f"- **{c.get('target')} killed** ({[w.get('pid') for w in c.get('killed') or []]}): app alive "
                         f"{c.get('app_alive')}, control API {c.get('control')}, tray {c.get('tray')}, recovered in "
                         f"place {c.get('recovered')} {c.get('recovery_log')}; then closed, WebView2 gone after "
                         f"{(c.get('close') or {}).get('webview_gone_s')} s.")
    cs = r.get("cold_start") or {}
    use = r.get("in_use") or {}
    lines += ["", "## Notes", "",
              f"- Cold starts (s): new profile {cs.get('first_s')}; window opens in the app running windowless "
              f"{cs.get('windowless_s')}; app not running {cs.get('not_running_s')}. The app's own count from its "
              f"start to the page load, with it not running (ms): {cs.get('in_process_ms')}.",
              "- Cold start ends at the window's first page load (navigation finished). The page mounts in its "
              "module script, which runs before that; data it fetches afterwards isn't counted.",
              f"- In use: {use.get('switches')} page changes over {(use.get('cost') or {}).get('seconds')} s "
              f"({', '.join(PAGES)}).",
              "- Memory is the private working set (USS) of the app process plus the WebView processes that belong "
              "to it (the speech worker excluded): reported, not bounded (decision astromech-01ft). CPU is the same "
              "processes' CPU time over the idle stretch, as a share of the whole machine (what Task Manager shows).",
              "- Baselines on windows-latest: the spike (astromech-wgtr) in the memory rows; a plain tray-icon "
              "process 1.9 MB; the two-process releases' core alone 6.2 MB and their window group 143 MB "
              "(App window cost run 42349317319). The spike had no pipeline, voice or caches.",
              "- A WebView2 process belongs to the app by descent, by `--webview-exe-name` or by its `EBWebView` "
              "folder, so an orphaned renderer still counts.",
              "", "Not covered: frame-time impact next to Elite (desktop and VR) and GPU use; that needs the game "
              "(astromech-7ck3). Pages show a fresh profile's data, not a played commander's. The tray icon isn't "
              "clicked and Raw Input isn't checked here (the onboarding smoke's kill-switch runs check it)."]
    return "\n".join(lines) + "\n"


# --- Win32 --------------------------------------------------------------------------------------

if os.name == "nt":
    import ctypes
    from ctypes import wintypes

    user32 = ctypes.WinDLL("user32", use_last_error=True)
    user32.PostMessageW.argtypes = [wintypes.HWND, wintypes.UINT, wintypes.WPARAM, wintypes.LPARAM]
    user32.GetWindowThreadProcessId.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.DWORD)]
    user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
    user32.GetWindowTextLengthW.argtypes = [wintypes.HWND]
    _ENUM = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)

    def find_window(pid: int, title: str) -> int | None:
        """A top-level window of `pid` with this title."""
        found: list[int] = []

        def cb(h, _):
            p = wintypes.DWORD()
            user32.GetWindowThreadProcessId(h, ctypes.byref(p))
            if p.value == pid:
                n = user32.GetWindowTextLengthW(h)
                buf = ctypes.create_unicode_buffer(n + 1)
                user32.GetWindowTextW(h, buf, n + 1)
                if buf.value == title:
                    found.append(h)
            return True

        user32.EnumWindows(_ENUM(cb), 0)
        return found[0] if found else None


# --- The app ------------------------------------------------------------------------------------


def webview_owner(name: str | None, cmdline: list[str] | None, descendant: bool, exe_name: str,
                  user_data: list[str]) -> list[str]:
    """Why a process is one of the app's WebView processes (none: it isn't). WebView2's processes
    are `msedgewebview2.exe`, WebKitGTK's `WebKit*`; one belongs to the app if it descends from
    it, if its command line has `--webview-exe-name=<exe_name>`, or if it names one of the
    `user_data` folders (lower case, backslashes)."""
    n = (name or "").lower()
    if n != "msedgewebview2.exe" and not n.startswith("webkit"):
        return []
    cmd = " ".join(cmdline or []).lower().replace("/", "\\")
    why = []
    if descendant:
        why.append("descendant")
    if f"--webview-exe-name={exe_name.lower()}" in cmd:
        why.append("webview-exe-name")
    if any(d and d in cmd for d in user_data):
        why.append("user-data")
    return why


class Profile:
    """A scratch per-user folder for the app."""

    def __init__(self, root: Path):
        self.root = root
        self.env = dict(os.environ)
        if os.name == "nt":
            self.env["LOCALAPPDATA"] = str(root)
            self.env["APPDATA"] = str(root / "Roaming")
        else:
            self.env["XDG_DATA_HOME"] = str(root)
            self.env["XDG_CONFIG_HOME"] = str(root / "config")
            self.env["XDG_CACHE_HOME"] = str(root / "cache")
            self.env["LOCALAPPDATA"] = str(root)  # where the keybinds look for Elite's Bindings folder
        self.log = root / "astromech" / "data" / "logs" / "astromech-core.log"
        self.endpoint = root / "astromech" / "data" / "core.json"
        # WebView2's profile: Tauri asks Windows for the local app data folder, which may not
        # follow LOCALAPPDATA, so both places count.
        real = os.environ.get("LOCALAPPDATA", "")
        self.webview_dirs = [str(Path(d) / IDENTIFIER / "EBWebView").lower().replace("/", "\\")
                             for d in (str(root), real) if d]


class App:
    """The app in a profile: its main process (`proc`), its control API and its log."""

    def __init__(self, exe: Path, prof: Profile, psutil):
        self.exe = exe.resolve()
        self.prof = prof
        self.ps = psutil
        self.proc: subprocess.Popen | None = None

    def start(self, args=()) -> subprocess.Popen:
        return subprocess.Popen([str(self.exe), *args], env=self.prof.env, stdout=subprocess.DEVNULL,
                                stderr=subprocess.DEVNULL)

    def launch(self, args, timeout: float) -> float:
        """Start the main instance and wait until its control API answers; the seconds it took."""
        try:
            self.prof.endpoint.unlink()
        except OSError:
            pass
        t0 = time.perf_counter()
        self.proc = self.start(args)
        while time.perf_counter() - t0 < timeout:
            ep = self.endpoint()
            if ep and ep.get("pid") == self.proc.pid and self.state() is not None:
                return time.perf_counter() - t0
            if self.proc.poll() is not None:
                raise RuntimeError(f"the app exited ({self.proc.returncode}) before its control API answered")
            time.sleep(0.05)
        raise RuntimeError(f"no control API within {timeout} s")

    def alive(self) -> bool:
        return self.proc is not None and self.proc.poll() is None

    def apps(self) -> list:
        """The app's processes in this profile (never another astromech on the machine)."""
        out = []
        for p in self.ps.process_iter(["name"]):
            try:
                if (p.info["name"] or "").lower().startswith("astromech-core"):
                    env = p.environ()
                    if str(self.prof.root) in (env.get("LOCALAPPDATA"), env.get("XDG_DATA_HOME")):
                        out.append(p)
            except self.ps.Error:
                pass
        return out

    def webviews(self) -> list[dict]:
        """The WebView processes that belong to the app, and why each does."""
        kids = set()
        if self.alive():
            try:
                kids = {c.pid for c in self.ps.Process(self.proc.pid).children(recursive=True)}
            except self.ps.Error:
                pass
        out = []
        for p in self.ps.process_iter(["name", "cmdline"]):
            try:
                why = webview_owner(p.info["name"], p.info["cmdline"], p.pid in kids, self.exe.name,
                                    self.prof.webview_dirs)
                if why:
                    kind = next((a.split("=", 1)[1] for a in (p.info["cmdline"] or []) if a.startswith("--type=")),
                                "browser" if p.info["name"].lower() == "msedgewebview2.exe" else p.info["name"])
                    out.append({"pid": p.pid, "type": kind, "why": why})
            except self.ps.Error:
                pass
        return out

    def endpoint(self) -> dict | None:
        try:
            return json.loads(self.prof.endpoint.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            return None

    def call(self, cmd: str, args=None, timeout=10.0) -> tuple[bool, object]:
        """One control API request: (ok, value or error)."""
        ep = self.endpoint()
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
        """`ui_state`: `{window, tray, page}`, or None if the control API didn't answer."""
        ok, v = self.call("ui_state")
        return v if ok and isinstance(v, dict) else None

    def lines(self, text: str) -> list[str]:
        try:
            return [l for l in self.prof.log.read_text(encoding="utf-8", errors="replace").splitlines() if text in l]
        except OSError:
            return []

    def page_loads(self) -> int:
        return len(self.lines("window: page loaded"))

    def tray(self) -> str:
        """`rect`, `fallback` (built, as the log says, but no rectangle), `none` or `no answer`."""
        v = self.state()
        if v is None:
            return "no answer"
        if v.get("tray"):
            return "rect"
        return "fallback" if self.lines("tray icon ready") else "none"

    def quit(self) -> None:
        """Quit the app (`--quit`), and make sure nothing of it is left in the profile."""
        try:
            subprocess.run([str(self.exe), "--quit"], env=self.prof.env, timeout=30)
        except (OSError, subprocess.TimeoutExpired):
            pass
        wait(lambda: not self.apps(), 10.0)
        self.kill_all()

    def kill_all(self) -> None:
        for p in self.apps():
            try:
                p.kill()
            except self.ps.Error:
                pass
        for w in self.webviews():
            try:
                self.ps.Process(w["pid"]).kill()
            except self.ps.Error:
                pass
        time.sleep(1)


def wait(pred, timeout: float, step=0.2):
    """Poll `pred` until it returns something truthy; that value, or the last one."""
    t0 = time.time()
    v = pred()
    while not v and time.time() - t0 < timeout:
        time.sleep(step)
        v = pred()
    return v


def group_uss_mb(app: App) -> tuple[float, int]:
    """The private working set of the app and its WebView processes now, and how many there are."""
    pids = ([app.proc.pid] if app.alive() else []) + [w["pid"] for w in app.webviews()]
    total = 0
    for pid in pids:
        try:
            total += app.ps.Process(pid).memory_full_info().uss
        except app.ps.Error:
            pass
    return round(total / 2**20, 1), len(pids)


def sample(app: App, seconds: float, action=None) -> dict:
    """Memory each second, and CPU time over `seconds`, of the app and its WebView processes;
    `action(i)` runs each second."""
    ps = app.ps
    first, last, mems, counts, seen, names = {}, {}, [], [], set(), set()
    t0 = time.time()
    i = 0
    while True:
        if action:
            action(i)
        i += 1
        wv = app.webviews()
        seen |= {w["pid"] for w in wv}
        pids = ([app.proc.pid] if app.alive() else []) + [w["pid"] for w in wv]
        total = 0
        for pid in pids:
            try:
                p = ps.Process(pid)
                total += p.memory_full_info().uss
                c = p.cpu_times()
                first.setdefault(pid, c.user + c.system)
                last[pid] = c.user + c.system
                names.add(p.name())
            except ps.Error:
                pass
        mems.append(total / 2**20)
        counts.append(len(pids))
        if time.time() - t0 >= seconds:
            break
        time.sleep(1)
    wall = max(time.time() - t0, 0.001)
    used = sum(last[p] - first[p] for p in last)
    return {
        "seconds": round(wall, 1),
        "uss_mb_mean": round(statistics.mean(mems), 1),
        "uss_mb_max": round(max(mems), 1),
        "cpu_core_pct": round(100 * used / wall, 2),
        "cpu_machine_pct": round(100 * used / wall / (os.cpu_count() or 1), 2),
        "processes_max": max(counts),
        "names": sorted(names),
        "webview_pids": sorted(seen),
    }


def second_start(app: App, route: str, timeout: float) -> dict:
    """A second start with `--route`: its exit code, and seconds from it to a new page load (None
    if none came within `timeout`)."""
    loads = app.page_loads()
    t0 = time.perf_counter()
    q = app.start(["--route", route])
    try:
        code = q.wait(timeout=30)
    except subprocess.TimeoutExpired:
        q.kill()
        code = "timeout"
    loaded = wait(lambda: app.page_loads() > loads, timeout - (time.perf_counter() - t0), 0.02)
    return {"exit_code": code, "page_load_s": round(time.perf_counter() - t0, 3) if loaded else None}


def close_window(app: App) -> dict:
    """WM_CLOSE to the window, as a click on X; then the window, WebView2, the app and the tray."""
    hwnd = find_window(app.proc.pid, TITLE) if app.alive() else None
    if not hwnd:
        return {"error": f"no window titled {TITLE!r}", "app_alive": app.alive(), "control": app.state() is not None,
                "window_closed_s": None, "webview_gone_s": None, "webview_left": app.webviews()}
    t0 = time.perf_counter()
    user32.PostMessageW(hwnd, 0x0010, 0, 0)  # WM_CLOSE
    closed = wait(lambda: (v := app.state()) and v.get("window") == "closed", 10.0)
    t_closed = round(time.perf_counter() - t0, 2)
    gone = wait(lambda: not app.webviews(), CLOSE_MAX_S - (time.perf_counter() - t0), 0.1)
    t_gone = round(time.perf_counter() - t0, 2)
    return {"window_closed_s": t_closed if closed else None, "webview_gone_s": t_gone if gone else None,
            "webview_left": app.webviews(), "app_alive": app.alive(), "control": app.state() is not None,
            "app_processes": len(app.apps()), "tray": app.tray(), "t0": t0}


def window_crash(app: App, target: str, timeout: float) -> dict:
    """Kill the window's renderer or its browser process. The app must live, the window must
    recover in place (what host.rs's ProcessFailed handler logs: a reload after the renderer
    exits; a rebuilt window and its own page load after the browser exits), and closing it must
    free WebView2."""
    r: dict = {"target": target}
    if (app.state() or {}).get("window") != "open":
        r["open"] = second_start(app, LIST_PAGE, timeout)
    time.sleep(3)
    victims = [w for w in app.webviews() if w["type"] == target]
    r["killed"] = victims
    if not victims:
        r.update(error=f"no {target} process belongs to the app", app_alive=app.alive(),
                 control=app.state() is not None, tray=app.tray())
        return r
    reloads, rebuilds, loads = (len(app.lines("window: reloading the page")), len(app.lines("window: rebuilt")),
                                app.page_loads())
    failures = len(app.lines("process failed"))
    for w in victims:
        try:
            app.ps.Process(w["pid"]).kill()
        except app.ps.Error:
            pass

    def recovered():
        v = app.state()
        if not v or v.get("window") != "open":
            return False
        if target == "renderer":
            return len(app.lines("window: reloading the page")) > reloads
        return len(app.lines("window: rebuilt")) > rebuilds and app.page_loads() > loads

    r["recovered"] = bool(wait(recovered, 20.0, 0.5))
    r["recovery_log"] = (app.lines("window: reloading the page")[reloads:] + app.lines("window: rebuilt")[rebuilds:])
    r["process_failed_log"] = app.lines("process failed")[failures:]
    r["app_alive"], r["control"], r["tray"] = app.alive(), app.state() is not None, app.tray()
    time.sleep(2)
    c = close_window(app)
    c.pop("t0", None)
    r["close"] = c
    return r


def measure(a, psutil) -> dict:
    root = Path(tempfile.mkdtemp(prefix="webview-cost-", dir=a.work))
    prof = Profile(root)
    app = App(a.app, prof, psutil)
    windows = os.name == "nt"
    skipped = {}
    if a.checks_only:
        skipped.update(page_changes="--checks-only", cold_start="--checks-only", cpu="--checks-only")
    if not windows:
        skipped.update(closes="Windows only (WM_CLOSE)", crashes="Windows only (WebView2)")
    r: dict = {"checks_only": a.checks_only, "skipped": skipped, "cold_start": {"windowless_s": []},
               "log": str(prof.log)}
    cs = r["cold_start"]
    try:
        # 1. Cold start with a new profile: the first start makes the WebView's profile.
        if not a.checks_only:
            cs["first_s"], cs["first_ms"] = cold_start(app, a.timeout)

        # 2. Windowless.
        wl = r["windowless"] = {}
        wl["started_s"] = round(app.launch(["--background"], a.timeout), 2)
        wait(lambda: app.tray() == "rect", 10.0, 0.5)
        wl["tray"] = app.tray()
        time.sleep(a.settle)
        wl["cost"] = sample(app, a.idle_seconds)
        wl["webview_pids"] = wl["cost"]["webview_pids"]
        wl["app_processes"] = len(app.apps())
        wl["alive"] = app.alive()

        # 3. Open, by a second start; idle on the page, then page changes.
        op = r["open"] = second_start(app, LIST_PAGE, a.timeout)
        if op["page_load_s"] is not None:
            cs["windowless_s"].append(op["page_load_s"])
        op["webview"] = wait(app.webviews, 10.0)
        time.sleep(a.settle)
        op["cost"] = sample(app, a.idle_seconds)
        op["webview_seen_idle"] = bool(op["cost"]["webview_pids"])
        if not a.checks_only:
            r["in_use"] = page_changes(app, a.use_seconds)

        # 4. Closed again, over CYCLES open and close cycles.
        if windows:
            r["closes"] = []
            for i in range(CYCLES):
                c: dict = {"cycle": i + 1}
                if i:
                    c["reopen"] = second_start(app, LIST_PAGE, a.timeout)
                    if c["reopen"]["page_load_s"] is not None:
                        cs["windowless_s"].append(c["reopen"]["page_load_s"])
                    time.sleep(2)
                c.update(close_window(app))
                t0 = c.pop("t0", None)
                if not a.checks_only and t0 is not None:
                    time.sleep(max(0.0, AFTER_CLOSE_S - (time.perf_counter() - t0)))
                    c["uss_after_mb"], c["processes_after"] = group_uss_mb(app)
                r["closes"].append(c)

            # 5. Window crash: the renderer, then the browser process.
            r["crashes"] = [window_crash(app, t, a.timeout) for t in ("renderer", "browser")]

        # 6. Quit with the window open.
        if (app.state() or {}).get("window") != "open":
            second_start(app, LIST_PAGE, a.timeout)
            time.sleep(2)
        ok, val = app.call("quit")
        try:
            code = app.proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            code = None
        wait(lambda: not app.webviews(), 5.0, 0.2)
        r["quit"] = {"reply": val if ok else f"error: {val}", "exit_code": code, "webview_left": app.webviews()}
        app.kill_all()

        # 7. Cold start with the app not running, on the profile it made.
        if not a.checks_only:
            cs["not_running_s"], cs["not_running_ms"] = cold_start(app, a.timeout)
            cs["in_process_ms"] = [cs["first_ms"], cs["not_running_ms"]]
    except Exception as e:  # the report says where it stopped; the gates with no data fail
        r["error"] = f"{type(e).__name__}: {e}"
    finally:
        app.quit()
    return r


def page_changes(app: App, seconds: float) -> dict:
    """A second start with `--route <page>` every 2 s over `seconds`: each hands its page to the
    running window and exits, and the app logs each page asked for."""
    loads, asked = app.page_loads(), len(app.lines(" asked for"))
    codes = []

    def switch(i):
        if i % 2 == 0:
            codes.append(app.start(["--route", PAGES[(i // 2) % len(PAGES)]]).wait(timeout=30))

    cost = sample(app, seconds, switch)
    return {"cost": cost, "switches": len(codes), "switch_exit_codes": sorted(set(codes)),
            # A second start that opened a window of its own instead would load a page of its own.
            "extra_windows": app.page_loads() - loads,
            "pages_received": len(app.lines(" asked for")) - asked, "window_alive": app.alive()}


def cold_start(app: App, timeout: float) -> tuple[float, int]:
    """The app not running: seconds from starting it (no arguments, so with its window) to the
    window's first page load, and the app's own count in ms. The app is quit after."""
    seen = app.page_loads()
    t0 = time.perf_counter()
    p = app.start()
    try:
        while time.perf_counter() - t0 < timeout:
            loads = app.lines("window: page loaded")
            if len(loads) > seen:
                ms = int(loads[-1].split("page loaded ")[1].split(" ms")[0])
                return round(time.perf_counter() - t0, 3), ms
            if p.poll() is not None:
                raise RuntimeError(f"the app exited ({p.returncode}) before its page loaded")
            time.sleep(0.01)
        raise RuntimeError(f"no page load within {timeout} s (log {app.prof.log})")
    finally:
        app.quit()


def git_commit() -> str:
    try:
        return os.environ.get("GITHUB_SHA") or subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
    except Exception:
        return "unknown"


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("--app", type=Path, required=True, help="astromech-core(.exe), built with the window")
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--work", type=Path, default=None, help="where the scratch profile goes (default: temp)")
    p.add_argument("--checks-only", action="store_true",
                   help="the process checks only: no page changes, waits after the closes or cold starts")
    p.add_argument("--settle", type=float, default=10.0, help="seconds to settle before each idle stretch")
    p.add_argument("--idle-seconds", type=float, default=30.0)
    p.add_argument("--use-seconds", type=float, default=30.0)
    p.add_argument("--timeout", type=float, default=60.0,
                   help="seconds a start may take to its page load or control API (the first WebView2 start "
                        "on a CI runner has taken 13 s)")
    a = p.parse_args(argv)
    # The Windows console is cp1252; the report isn't.
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    import psutil

    a.out.mkdir(parents=True, exist_ok=True)
    started = time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime())
    results = measure(a, psutil)
    rep = {
        "webview": "WebView2" if os.name == "nt" else "WebKitGTK",
        "host": f"{platform.system()} {platform.release()} ({platform.machine()}, {os.cpu_count()} CPUs)",
        "commit": git_commit(),
        "started": started,
        "checks": judge(results),
        "results": results,
    }
    (a.out / "report.json").write_text(json.dumps(rep, indent=1, default=str), encoding="utf-8")
    try:
        shutil.copyfile(results["log"], a.out / "astromech-core.log")
    except OSError:
        pass
    md = markdown(rep)
    (a.out / "report.md").write_text(md, encoding="utf-8")
    print(md)
    return 1 if failed(rep["checks"]) else 0


if __name__ == "__main__":
    sys.exit(main())
