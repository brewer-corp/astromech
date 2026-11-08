#!/usr/bin/env python3
"""Tests for the app window cost harness's judging and report (scripts/webview_cost.py). Runs
anywhere: python3 scripts/test_webview_cost.py"""

import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import webview_cost as w  # noqa: E402


def cost(mb=120.0, cpu=0.25, pids=(11, 12)):
    return {"seconds": 30.0, "uss_mb_mean": mb - 2, "uss_mb_max": mb, "cpu_core_pct": cpu * 4,
            "cpu_machine_pct": cpu, "processes_max": 1 + len(pids), "names": ["astromech-core.exe"],
            "webview_pids": list(pids)}


def close(cycle, gone=2.3, tray="rect"):
    return {"cycle": cycle, "window_closed_s": 0.24, "webview_gone_s": gone, "webview_left": [], "app_alive": True,
            "control": True, "app_processes": 1, "tray": tray, "uss_after_mb": 9.6 - cycle / 2, "processes_after": 1,
            **({"reopen": {"exit_code": 0, "page_load_s": 0.6}} if cycle > 1 else {})}


def crash(target):
    return {"target": target, "killed": [{"pid": 20, "type": target, "why": ["descendant"]}], "recovered": True,
            "recovery_log": [], "app_alive": True, "control": True, "tray": "rect",
            "close": {"window_closed_s": 0.3, "webview_gone_s": 2.0, "webview_left": [], "app_alive": True,
                      "control": True, "tray": "rect"}}


def windows_run():
    """A Windows run within every gate."""
    return {
        "checks_only": False, "skipped": {},
        "cold_start": {"first_s": 5.0, "windowless_s": [0.62, 0.6, 0.7, 0.65, 0.66], "not_running_s": 0.68,
                       "in_process_ms": [4000, 650]},
        "windowless": {"tray": "rect", "cost": cost(3.4, 0.0, ()), "webview_pids": [], "app_processes": 1,
                       "alive": True},
        "open": {"exit_code": 0, "page_load_s": 0.62, "webview": [{"pid": 11}], "cost": cost(),
                 "webview_seen_idle": True},
        "in_use": {"cost": cost(160.0, 8.0), "switches": 15, "switch_exit_codes": [0], "extra_windows": 0,
                   "pages_received": 15, "window_alive": True},
        "closes": [close(i) for i in range(1, 6)],
        "crashes": [crash("renderer"), crash("browser")],
        "quit": {"window_before": "open", "webview_before": [{"pid": 11}], "reply": None, "exit_code": 0,
                 "webview_left": []},
    }


def verdicts(r):
    return {c["id"]: c["verdict"] for c in w.judge(r)}


GATES = ["windowless", "open", "page_changes", "close", "app_alive", "window_crash", "quit", "cold_start",
         "idle_cpu_windowless", "idle_cpu_open"]


class Judge(unittest.TestCase):
    def test_a_passing_run(self):
        v = verdicts(windows_run())
        self.assertEqual({k: v[k] for k in GATES}, {k: w.PASS for k in GATES})
        self.assertEqual(v["tray"], w.PASS)
        self.assertFalse(w.failed(w.judge(windows_run())))

    def test_memory_is_reported_with_no_pass_mark(self):
        r = windows_run()
        r["windowless"]["cost"]["uss_mb_max"] = 900.0
        r["open"]["cost"]["uss_mb_max"] = 2000.0
        v = verdicts(r)
        for k in ("memory_windowless", "memory_open", "memory_closed"):
            self.assertEqual(v[k], w.REPORTED, k)
        self.assertFalse(w.failed(w.judge(r)))
        self.assertFalse(hasattr(w, "IDLE_MB_MAX"))

    def test_a_webview2_process_left_after_a_close_fails(self):
        r = windows_run()
        r["closes"][2].update(webview_gone_s=None, webview_left=[{"pid": 99, "type": "renderer", "why": ["webview-exe-name"]}])
        rows = w.judge(r)
        self.assertEqual(verdicts(r)["close"], w.FAIL)
        self.assertTrue(w.failed(rows))
        self.assertIn("left [99]", next(c for c in rows if c["id"] == "close")["value"])

    def test_a_close_must_free_webview2_within_15_s(self):
        r = windows_run()
        r["closes"][0]["webview_gone_s"] = 15.0
        self.assertEqual(verdicts(r)["close"], w.PASS)
        r["closes"][0]["webview_gone_s"] = 15.01
        self.assertEqual(verdicts(r)["close"], w.FAIL)

    def test_every_cycle_must_run(self):
        r = windows_run()
        r["closes"] = r["closes"][:4]
        self.assertEqual(verdicts(r)["close"], w.FAIL)
        r["closes"] = []
        self.assertEqual(verdicts(r)["close"], w.FAIL)

    def test_webview2_windowless_fails(self):
        r = windows_run()
        r["windowless"]["webview_pids"] = [40]
        self.assertEqual(verdicts(r)["windowless"], w.FAIL)
        r = windows_run()
        r["windowless"]["app_processes"] = 2
        self.assertEqual(verdicts(r)["windowless"], w.FAIL)
        r = windows_run()
        del r["windowless"]["webview_pids"]
        self.assertEqual(verdicts(r)["windowless"], w.FAIL)

    def test_the_app_must_outlive_each_close_and_crash(self):
        for where in ("closes", "crashes"):
            r = windows_run()
            r[where][1]["app_alive"] = False
            self.assertEqual(verdicts(r)["app_alive"], w.FAIL, where)
            r = windows_run()
            r[where][0]["control"] = False
            self.assertEqual(verdicts(r)["app_alive"], w.FAIL, where)

    def test_a_crashed_windows_close_must_pass(self):
        # Finding astromech-k6f1: the crash isn't a pass if WebView2 lingers after its close.
        r = windows_run()
        r["crashes"][1]["close"]["webview_gone_s"] = None
        self.assertEqual(verdicts(r)["window_crash"], w.FAIL)
        r = windows_run()
        del r["crashes"][0]["close"]
        self.assertEqual(verdicts(r)["window_crash"], w.FAIL)

    def test_a_crash_must_recover_in_place_and_kill_something(self):
        r = windows_run()
        r["crashes"][0]["recovered"] = False
        self.assertEqual(verdicts(r)["window_crash"], w.FAIL)
        r = windows_run()
        r["crashes"][1]["killed"] = []
        self.assertEqual(verdicts(r)["window_crash"], w.FAIL)
        r = windows_run()
        r["crashes"] = r["crashes"][:1]
        self.assertEqual(verdicts(r)["window_crash"], w.FAIL)

    def test_the_tray_fallback_is_degraded_not_a_pass(self):
        # No notification area all along: every reading is the fallback.
        r = windows_run()
        r["windowless"]["tray"] = "fallback"
        for e in r["closes"] + r["crashes"]:
            e["tray"] = "fallback"
        rows = w.judge(r)
        self.assertEqual(verdicts(r)["tray"], w.DEGRADED)
        self.assertFalse(w.failed(rows))
        self.assertIn("**DEGRADED**", w.markdown(report(r)))
        r["closes"][4]["tray"] = "none"
        self.assertEqual(verdicts(r)["tray"], w.FAIL)
        self.assertEqual(w.tray_verdict([]), w.FAIL)

    def test_a_tray_that_loses_its_rectangle_fails(self):
        # Finding astromech-m6f9: rect at first, then fallback, is a lost icon, not a degradation.
        r = windows_run()
        r["closes"][3]["tray"] = "fallback"
        r["closes"][4]["tray"] = "fallback"
        self.assertEqual(verdicts(r)["tray"], w.FAIL)
        self.assertEqual(w.tray_verdict(["fallback", "fallback"]), w.DEGRADED)

    def test_a_reopen_that_doesnt_load_fails_the_close(self):
        # Finding astromech-zocz: a reopen that timed out or loaded nothing isn't a passing cycle.
        for bad in ({"exit_code": "timeout", "page_load_s": None}, {"exit_code": 0, "page_load_s": None},
                    {"exit_code": 1, "page_load_s": 0.6}, None):
            r = windows_run()
            r["closes"][2]["reopen"] = bad
            self.assertEqual(verdicts(r)["close"], w.FAIL, bad)
        # The reviewer's case: --checks-only with closes[2]'s reopen timed out fails the run.
        r = windows_run()
        r["checks_only"] = True
        r["skipped"] = {"page_changes": "--checks-only", "cold_start": "--checks-only", "cpu": "--checks-only"}
        r["closes"][2]["reopen"] = {"exit_code": "timeout", "page_load_s": None}
        self.assertTrue(w.failed(w.judge(r)))

    def test_quit_needs_the_window_open_before_it(self):
        for key, bad in (("window_before", "closed"), ("window_before", None), ("webview_before", [])):
            r = windows_run()
            r["quit"][key] = bad
            self.assertEqual(verdicts(r)["quit"], w.FAIL, key)

    def test_windows_needs_five_cold_start_samples(self):
        r = windows_run()
        r["cold_start"]["windowless_s"] = [0.6, 0.6, 0.6, 0.6]
        self.assertEqual(verdicts(r)["cold_start"], w.FAIL)

    def test_quit(self):
        for key, bad in (("exit_code", 1), ("exit_code", None), ("webview_left", [{"pid": 3}])):
            r = windows_run()
            r["quit"][key] = bad
            self.assertEqual(verdicts(r)["quit"], w.FAIL, key)

    def test_open(self):
        for key, bad in (("exit_code", 1), ("page_load_s", None), ("webview", [])):
            r = windows_run()
            r["open"][key] = bad
            self.assertEqual(verdicts(r)["open"], w.FAIL, key)

    def test_page_changes_must_reach_the_window(self):
        for key, bad in (("extra_windows", 2), ("pages_received", 14), ("switch_exit_codes", [0, 1]),
                         ("window_alive", False), ("switches", 0)):
            r = windows_run()
            r["in_use"][key] = bad
            self.assertEqual(verdicts(r)["page_changes"], w.FAIL, key)

    def test_cold_start(self):
        r = windows_run()
        r["cold_start"]["windowless_s"] = [1.9, 2.0, 2.4, 2.1, 2.2]
        self.assertEqual(verdicts(r)["cold_start"], w.FAIL)
        r = windows_run()
        r["cold_start"]["not_running_s"] = 2.1
        self.assertEqual(verdicts(r)["cold_start"], w.FAIL)
        r = windows_run()
        r["cold_start"]["not_running_s"] = None
        self.assertEqual(verdicts(r)["cold_start"], w.FAIL)
        # The first start makes the WebView profile, once per install: not judged.
        r = windows_run()
        r["cold_start"]["first_s"] = 13.0
        self.assertEqual(verdicts(r)["cold_start"], w.PASS)

    def test_idle_cpu_windowless_and_open(self):
        for where, gid in (("windowless", "idle_cpu_windowless"), ("open", "idle_cpu_open")):
            r = windows_run()
            r[where]["cost"]["cpu_machine_pct"] = 1.0
            self.assertEqual(verdicts(r)[gid], w.PASS)
            r[where]["cost"]["cpu_machine_pct"] = 1.01
            self.assertEqual(verdicts(r)[gid], w.FAIL)
        r = windows_run()
        r["open"]["webview_seen_idle"] = False  # nothing measured is not a pass
        self.assertEqual(verdicts(r)["idle_cpu_open"], w.FAIL)

    def test_a_run_that_stopped_early_fails(self):
        r = {"checks_only": False, "skipped": {}, "cold_start": {"windowless_s": []},
             "error": "RuntimeError: no control API within 60 s"}
        rows = w.judge(r)
        self.assertEqual({c["id"]: c["verdict"] for c in rows if c["id"] in GATES}, {k: w.FAIL for k in GATES})
        self.assertIn("stopped early", w.markdown(report(r)))

    def test_checks_only_judges_the_checks_and_not_the_cost(self):
        r = windows_run()
        r["checks_only"] = True
        r["skipped"] = {"page_changes": "--checks-only", "cold_start": "--checks-only", "cpu": "--checks-only"}
        del r["in_use"]
        r["cold_start"] = {"windowless_s": [13.0]}
        r["open"]["cost"]["cpu_machine_pct"] = 5.0
        v = verdicts(r)
        for k in ("page_changes", "cold_start", "idle_cpu_windowless", "idle_cpu_open"):
            self.assertEqual(v[k], w.NOT_RUN, k)
        for k in ("windowless", "open", "close", "app_alive", "window_crash", "quit"):
            self.assertEqual(v[k], w.PASS, k)
        self.assertFalse(w.failed(w.judge(r)))
        r["closes"][0]["webview_gone_s"] = None
        self.assertTrue(w.failed(w.judge(r)))

    def test_linux_reports_the_windows_only_steps_as_not_run(self):
        r = windows_run()
        r["skipped"] = {"closes": "Windows only (WM_CLOSE)", "crashes": "Windows only (WebView2)"}
        del r["closes"], r["crashes"]
        r["cold_start"]["windowless_s"] = [0.9]
        r["windowless"]["tray"] = "fallback"
        v = verdicts(r)
        for k in ("close", "app_alive", "window_crash", "memory_closed"):
            self.assertEqual(v[k], w.NOT_RUN, k)
        self.assertEqual(v["tray"], w.DEGRADED)
        self.assertFalse(w.failed(w.judge(r)))


class Ownership(unittest.TestCase):
    EXE = "astromech-core.exe"
    DATA = ["c:\\users\\r\\appdata\\local\\io.github.spadaval.astromech\\ebwebview"]

    def owner(self, name, cmd, descendant=False):
        return w.webview_owner(name, cmd, descendant, self.EXE, self.DATA)

    def test_descent(self):
        self.assertEqual(self.owner("msedgewebview2.exe", [], True), ["descendant"])
        self.assertEqual(self.owner("WebKitWebProcess", ["/usr/lib/WebKitWebProcess", "13"], True), ["descendant"])

    def test_an_orphaned_renderer_still_counts(self):
        cmd = ["msedgewebview2.exe", "--type=renderer", "--webview-exe-name=astromech-core.exe"]
        self.assertEqual(self.owner("msedgewebview2.exe", cmd), ["webview-exe-name"])
        cmd = ["msedgewebview2.exe", "--user-data-dir=C:/Users/r/AppData/Local/io.github.spadaval.astromech/EBWebView"]
        self.assertEqual(self.owner("msedgewebview2.exe", cmd), ["user-data"])

    def test_others_dont(self):
        self.assertEqual(self.owner("msedgewebview2.exe", ["--webview-exe-name=Teams.exe"]), [])
        self.assertEqual(self.owner("astromech-voice-worker.exe", [], True), [])
        self.assertEqual(self.owner("msedge.exe", ["--webview-exe-name=astromech-core.exe"], True), [])


def report(r):
    return {"webview": "WebView2", "host": "test", "commit": "abc", "started": "now", "checks": w.judge(r),
            "results": r}


class Report(unittest.TestCase):
    def test_a_passing_report_shows_each_cycle(self):
        md = w.markdown(report(windows_run()))
        self.assertNotIn("**FAIL**", md)
        self.assertIn("## Open and closed again (5 cycles)", md)
        for c in windows_run()["closes"]:
            self.assertIn(f"| {c['cycle']} |", md)
            self.assertIn(f"| {c['uss_after_mb']} |", md)
        self.assertIn("## Window crash", md)
        self.assertIn("astromech-7ck3", md)
        self.assertIn("astromech-01ft", md)
        self.assertEqual(md.count("| reported |"), 3)

    def test_a_failing_report(self):
        r = windows_run()
        r["closes"][4]["webview_gone_s"] = None
        r["quit"]["exit_code"] = 1
        md = w.markdown(report(r))
        self.assertEqual(md.count("**FAIL**"), 2)

    def test_the_report_doesnt_change_the_results(self):
        r = windows_run()
        before = copy.deepcopy(r)
        w.markdown(report(r))
        self.assertEqual(r, before)


if __name__ == "__main__":
    unittest.main()
