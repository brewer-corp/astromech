#!/usr/bin/env python3
"""WebView cost harness: what the app window costs, against the S11 budgets
(docs/ARCHITECTURE.md §14 Phase 0 and §18.1).

    python scripts/webview_cost.py --app target/release/astromech-core.exe --out webview-report

Runs the app (`astromech-core`: the core, the tray and the window in one process; WebView2 on
Windows, WebKitGTK on Linux) in a scratch profile and measures:
- cold start: to the window's first page load (the app logs `window: page loaded` in
  `astromech-core.log`). The first start makes the WebView's profile. The later ones open the
  window in the app running windowless (in the tray), as a player usually does: the app is
  started with `--background`, then a second start with `--route` opens the window. The budget
  is judged on the later ones' median, and on a start with the app not running at all.
- memory: the private working set (USS) summed over the app process and its WebView processes
  (the speech worker excluded), idle on a list-heavy page, and its peak while moving between
  pages; and the app windowless, before the window opens.
- CPU: the same processes' CPU time over an idle stretch and while moving between pages, as a
  share of the whole machine (what Task Manager shows) and of one core.
Pages are changed the way the tray does it: a second start with `--route <page>` hands the page
to the running window. The page data is whatever a fresh profile has (no game data), so lists
are short apart from the keybinds; frame-time impact next to Elite needs the game and a GPU
(astromech-7ck3). The fuller rework (open, closed again, no WebView left) is astromech-xz7i.
Needs psutil. Exit status 1 when a budget is missed.
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import statistics
import subprocess
import sys
import json
import tempfile
import time
from pathlib import Path

IDLE_MB_MAX = 250.0
IDLE_CPU_MAX = 1.0  # % of the machine
COLD_START_MAX_S = 2.0
LIST_PAGE = "keybinds"
PAGES = ["keybinds", "materials", "engineering", "trade", "logbook", "exploration", "outfitting", "settings"]


def judge(r: dict) -> list[dict]:
    """The S11 budgets against the measurements in `r` (see `measure`)."""
    cs = r["cold_start"]
    later = cs["later_s"]
    med = statistics.median(later) if later else None
    not_running = cs.get("app_not_running_s")
    idle, use = r["idle"], r["in_use"]
    # The idle numbers count only if the window and its WebView were there to measure.
    measured = idle.get("window_alive", False) and idle.get("webview_seen", False) and idle["uss_mb_max"] > 20
    return [
        {"id": "idle_memory", "budget": f"< {IDLE_MB_MAX:g} MB private working set, idle on a list-heavy page",
         "value": f"{idle['uss_mb_mean']} MB mean, {idle['uss_mb_max']} MB max (in use: peak {use['uss_mb_max']} MB)",
         "pass": measured and idle["uss_mb_max"] < IDLE_MB_MAX},
        {"id": "idle_cpu", "budget": f"<= {IDLE_CPU_MAX:g}% CPU idle",
         "value": f"{idle['cpu_machine_pct']}% of the machine ({idle['cpu_core_pct']}% of one core); "
                  f"in use {use['cpu_machine_pct']}%",
         "pass": measured and idle["cpu_machine_pct"] <= IDLE_CPU_MAX},
        {"id": "cold_start", "budget": f"< {COLD_START_MAX_S:g} s cold start",
         "value": f"median {med if med is None else round(med, 2)} s over {len(later)} window opens in the app "
                  f"running windowless; {not_running} s with the app not running "
                  f"(first start, new profile: {cs['first_s']} s)",
         "pass": med is not None and med < COLD_START_MAX_S and not_running is not None
                 and not_running < COLD_START_MAX_S},
        # The harness's own check: the page changes reached the running window.
        {"id": "page_changes", "budget": "harness check: every page change reached the running window",
         "value": f"{use.get('switches', 0)} asked, {use.get('pages_received', 0)} received, "
                  f"{use.get('extra_windows', 0)} extra windows, exit codes {use.get('switch_exit_codes')}; "
                  f"window and WebView there throughout: {measured and use.get('window_alive', False)}",
         "pass": measured and use.get("window_alive", False) and use.get("switches", 0) > 0
                 and use.get("pages_received") == use.get("switches") and use.get("extra_windows", 0) == 0
                 and use.get("switch_exit_codes") == [0]},
    ]


def markdown(rep: dict) -> str:
    lines = ["# App window cost (S11 budgets)", "",
             f"{rep['webview']} on {rep['host']}, commit `{rep['commit']}`, {rep['started']}.", "",
             "| Budget | Measured | Verdict |", "|---|---|---|"]
    for c in rep["budgets"]:
        lines.append(f"| {c['budget']} | {c['value']} | {'PASS' if c['pass'] else '**FAIL**'} |")
    r = rep["results"]
    lines += ["", f"- Cold starts (s): first {r['cold_start']['first_s']}, then {r['cold_start']['later_s']}; "
              f"the app's own count from its start to the page load, with it not running (ms): "
              f"{r['cold_start']['in_process_ms']}.",
              f"- Idle on `{LIST_PAGE}` for {r['idle']['seconds']} s: {r['idle']['processes']} processes "
              f"({', '.join(r['idle']['names'])}).",
              "- Cold start ends at the window's first page load (navigation finished). The page mounts in its "
              "module script, which runs before that; data it fetches afterwards isn't counted. With the app "
              "not running, it includes the app's own start; in the app running windowless, it starts at the "
              "second start that asks for the window.",
              f"- The app windowless before the window opens: {r.get('windowless', {}).get('uss_mb_mean')} MB "
              f"mean private working set, {r.get('windowless', {}).get('processes')} processes.",
              f"- In use: {r['in_use']['switches']} page changes over {r['in_use']['seconds']} s "
              f"({', '.join(PAGES)}); peak {r['in_use']['uss_mb_max']} MB; second starts exited with "
              f"{r['in_use'].get('switch_exit_codes')}, extra windows opened: {r['in_use'].get('extra_windows')}.",
              "- The app process and its WebView processes are measured together: the core is in the app.",
              "", "Not covered: frame-time impact next to Elite (desktop and VR) and GPU use; that needs the game "
              "(astromech-7ck3). Pages show a fresh profile's data, not a played commander's."]
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------------------------


def tree(app, psutil) -> list:
    """The app process and every process under it except the speech worker."""
    out = [app]
    try:
        kids = app.children()
    except psutil.Error:
        return out
    stack = list(kids)
    while stack:
        p = stack.pop()
        try:
            if p.name().lower().startswith("astromech-voice"):
                continue
            out.append(p)
            stack.extend(p.children())
        except psutil.Error:
            pass
    return out


def our_apps(prof: "Profile", psutil) -> list:
    """The app's processes in this profile (never another astromech on the machine)."""
    out = []
    for p in psutil.process_iter(["name"]):
        try:
            if (p.info["name"] or "").lower().startswith("astromech-core"):
                env = p.environ()
                if str(prof.root) in (env.get("LOCALAPPDATA"), env.get("XDG_DATA_HOME")):
                    out.append(p)
        except psutil.Error:
            pass
    return out


def uss_mb(procs, psutil) -> float:
    t = 0
    for p in procs:
        try:
            t += p.memory_full_info().uss
        except psutil.Error:
            pass
    return t / 2**20


def cpu_s(procs, psutil) -> float:
    t = 0.0
    for p in procs:
        try:
            c = p.cpu_times()
            t += c.user + c.system
        except psutil.Error:
            pass
    return t


def window(app, psutil, seconds: float, action=None) -> dict:
    """Sample memory every second and CPU time over `seconds`; `action(i)` runs each second."""
    procs = tree(app, psutil)
    c0, t0 = cpu_s(procs, psutil), time.time()
    mems, i = [], 0
    while time.time() - t0 < seconds:
        if action:
            action(i)
        i += 1
        time.sleep(1)
        procs = tree(app, psutil)
        mems.append(uss_mb(procs, psutil))
    wall = time.time() - t0
    used = cpu_s(procs, psutil) - c0
    names = sorted({p.name() for p in procs if p.is_running()})
    webview = any(n.lower().startswith(("msedgewebview2", "webkit")) for n in names)
    return {
        "seconds": round(wall, 1),
        "uss_mb_mean": round(statistics.mean(mems), 1),
        "uss_mb_max": round(max(mems), 1),
        "cpu_core_pct": round(100 * used / wall, 2),
        "cpu_machine_pct": round(100 * used / wall / (os.cpu_count() or 1), 2),
        "processes": len(procs),
        "names": names,
        "webview_seen": webview,
    }


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

    def lines(self, text: str) -> list[str]:
        try:
            return [l for l in self.log.read_text(encoding="utf-8", errors="replace").splitlines() if text in l]
        except OSError:
            return []

    def page_loads(self) -> list[str]:
        return self.lines("window: page loaded")

    def pages_asked(self) -> list[str]:
        return self.lines(" asked for")


def start(app_exe: Path, prof: Profile, args=()) -> subprocess.Popen:
    return subprocess.Popen([str(app_exe), *args], env=prof.env, stdout=subprocess.DEVNULL,
                            stderr=subprocess.DEVNULL)


def quit_app(app_exe: Path, prof: Profile, psutil) -> None:
    """Quit the app (`--quit`), and make sure no process of it is left in the profile."""
    subprocess.run([str(app_exe), "--quit"], env=prof.env, timeout=30)
    end = time.time() + 10
    while our_apps(prof, psutil) and time.time() < end:
        time.sleep(0.2)
    for p in our_apps(prof, psutil):
        p.kill()
    time.sleep(1)


def wait_load(prof: Profile, seen: int, t0: float, timeout: float, p: subprocess.Popen | None = None) -> tuple[float, int]:
    """Seconds from `t0` to a page load after the first `seen`, and the app's own count in ms."""
    while time.perf_counter() - t0 < timeout:
        loads = prof.page_loads()
        if len(loads) > seen:
            ms = int(loads[-1].split("page loaded ")[1].split(" ms")[0])
            return time.perf_counter() - t0, ms
        if p is not None and p.poll() is not None:
            raise RuntimeError(f"the app exited ({p.returncode}) before its page loaded")
        time.sleep(0.01)
    raise RuntimeError(f"no page load within {timeout} s (log {prof.log})")


def wait_endpoint(prof: Profile, p: subprocess.Popen, timeout: float) -> None:
    """Until the app started as `p` has published its control endpoint."""
    end = time.time() + timeout
    while time.time() < end:
        try:
            if json.loads(prof.endpoint.read_text(encoding="utf-8")).get("pid") == p.pid:
                return
        except (OSError, ValueError):
            pass
        if p.poll() is not None:
            raise RuntimeError(f"the app exited ({p.returncode}) before its control endpoint was there")
        time.sleep(0.05)
    raise RuntimeError(f"no control endpoint within {timeout} s")


def cold_start(app_exe: Path, prof: Profile, psutil, timeout: float) -> tuple[float, int]:
    """The app not running: seconds from starting it to its window's first page load, and its own
    count in ms. The app is quit after."""
    seen = len(prof.page_loads())
    t0 = time.perf_counter()
    p = start(app_exe, prof)
    try:
        return wait_load(prof, seen, t0, timeout, p)
    finally:
        quit_app(app_exe, prof, psutil)


def windowless_start(app_exe: Path, prof: Profile, psutil, timeout: float, seconds: float) -> tuple[float, int, dict]:
    """The app running windowless (`--background`), measured for `seconds`, then a second start
    with `--route` opens the window: seconds from that start to the page load, the app's own count
    in ms, and the windowless measurement. The app is quit after."""
    p = start(app_exe, prof, ["--background"])
    try:
        wait_endpoint(prof, p, timeout)
        idle = window(psutil.Process(p.pid), psutil, seconds) if seconds else {}
        seen = len(prof.page_loads())
        t0 = time.perf_counter()
        start(app_exe, prof, ["--route", LIST_PAGE]).wait(timeout=30)
        dt, ms = wait_load(prof, seen, t0, timeout, p)
        return dt, ms, idle
    finally:
        quit_app(app_exe, prof, psutil)


def measure(a, psutil) -> dict:
    root = Path(tempfile.mkdtemp(prefix="webview-cost-", dir=a.work))
    prof = Profile(root)
    # The first start makes the WebView's profile, with the app not running.
    first = cold_start(a.app, prof, psutil, a.timeout)
    # The window opened in the app running in the tray, as a player usually opens it; the
    # first of these also measures the app windowless.
    later = [windowless_start(a.app, prof, psutil, a.timeout, a.idle_seconds if i == 0 else 0)
             for i in range(a.starts - 1)]
    windowless = later[0][2] if later else {}
    # The app not running, on the profile it made.
    not_running = cold_start(a.app, prof, psutil, a.timeout)
    loads_before = len(prof.page_loads())
    app = start(a.app, prof, ["--route", LIST_PAGE])
    try:
        sp = psutil.Process(app.pid)
        end = time.time() + a.timeout
        while time.time() < end and len(prof.page_loads()) <= loads_before:
            time.sleep(0.05)
        time.sleep(a.settle)
        idle = window(sp, psutil, a.idle_seconds)
        idle["window_alive"] = sp.is_running()

        loads_before, asked_before = len(prof.page_loads()), len(prof.pages_asked())
        codes = []

        def switch(i):
            # A second start hands the page to the running window and exits.
            if i % 2 == 0:
                codes.append(start(a.app, prof, ["--route", PAGES[(i // 2) % len(PAGES)]]).wait(timeout=30))

        in_use = window(sp, psutil, a.use_seconds, switch)
        in_use["switches"] = len(codes)
        in_use["switch_exit_codes"] = sorted(set(codes))
        # A second start that opened a window of its own instead would load a page of its own.
        in_use["extra_windows"] = len(prof.page_loads()) - loads_before
        # The running app logs each page a second start asks its window for.
        in_use["pages_received"] = len(prof.pages_asked()) - asked_before
        in_use["window_alive"] = sp.is_running()
    finally:
        quit_app(a.app, prof, psutil)
    return {
        "cold_start": {"first_s": round(first[0], 3), "later_s": [round(s, 3) for s, _, _ in later],
                       # The app's own count runs from its process start, so only for starts with it not running.
                       "in_process_ms": [first[1], not_running[1]],
                       "app_not_running_s": round(not_running[0], 3)},
        "windowless": windowless,
        "idle": idle,
        "in_use": in_use,
    }


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
    p.add_argument("--starts", type=int, default=6,
                   help="cold starts: the first makes the profile, the rest open the window in the app windowless")
    p.add_argument("--settle", type=float, default=10.0, help="seconds after the page loads before idle")
    p.add_argument("--idle-seconds", type=float, default=30.0)
    p.add_argument("--use-seconds", type=float, default=30.0)
    p.add_argument("--timeout", type=float, default=60.0)
    a = p.parse_args(argv)
    import psutil

    a.out.mkdir(parents=True, exist_ok=True)
    started = time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime())
    results = measure(a, psutil)
    rep = {
        "webview": "WebView2" if os.name == "nt" else "WebKitGTK",
        "host": f"{platform.system()} {platform.release()} ({platform.machine()}, {os.cpu_count()} CPUs)",
        "commit": git_commit(),
        "started": started,
        "budgets": judge(results),
        "results": results,
    }
    (a.out / "report.json").write_text(json.dumps(rep, indent=1), encoding="utf-8")
    md = markdown(rep)
    (a.out / "report.md").write_text(md, encoding="utf-8")
    print(md)
    return 0 if all(c["pass"] for c in rep["budgets"]) else 1


if __name__ == "__main__":
    sys.exit(main())
