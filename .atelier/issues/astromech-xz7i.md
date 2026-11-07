---
created_at: "2026-11-07T11:57:35.951210811+00:00"
id: "astromech-xz7i"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-v3u4"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "webview_cost.py measures the one app windowless, open and closed again, and fails on a WebView2 process left behind"
updated_at: "2026-11-07T22:12:26.144400043+00:00"
---

## Goal

`scripts/webview_cost.py` and `.github/workflows/webview-cost.yml` measure the one app as a player uses it: windowless, with the window open, and closed again. They fail the run if any WebView2 process outlives the window. Memory is reported, not bounded (decision astromech-01ft). With this child the spike's probe is folded into the harness, so success criterion one-process is shown by one workflow.

## Done when

- [ ] webview_cost.py takes `--app <astromech-core(.exe)>` and, in a scratch profile, measures the following. A WebView2 process belongs to the app as in the probe: it is a descendant, or its command line has `--webview-exe-name=<app exe>` or names the profile's `EBWebView` folder. So an orphaned renderer still counts, and today's `tree()` (webview_cost.py:105-123), which follows descent only, is replaced.
  - **Windowless:** started with `--background`, then left to settle. It records the app's private working set and CPU over the idle stretch, and that no WebView2 process belongs to it.
  - **Open:** a second start with `--route keybinds`. It records the group's private working set and CPU idle on that page, then over page changes (`PAGES`, as now), and the page changes each reach the window.
  - **Closed again:** `WM_CLOSE` to the window. It records the time until no WebView2 process belonging to the app is left, and the app's private working set after 30 s. This runs over 5 cycles, and the report shows each cycle's numbers so a leak would show.
  - **Cold start:** from a second start to the page-load line, with the app running windowless (the median of 5) and with it not running.
  - **Window crash:** the probe's check 6, so the app survives its renderer and its browser process being killed.
- [x] Its `judge` gates:
  - no WebView2 process windowless;
  - none within 15 s of each close;
  - the app alive after each crash and close;
  - cold start under 2 s;
  - idle CPU at most 1% of the machine, both windowless and open.

  Memory rows are reported with no pass mark, and `IDLE_MB_MAX` goes.

  scripts/test_webview_cost.py covers `judge` and the report for passing and failing runs, including a WebView2 process left after a close.
- [ ] webview-cost.yml builds the one app and runs the harness: Windows gates and Linux reports, as now. It runs on PRs that touch app/src-tauri/**, the app, crates/astromech-core/src/{app,tray,cli}.rs or the harness. Its header comment describes the one process.
- [x] scripts/one_process_probe.py is reduced to its restart mode, which astromech-dl53's job uses, and its other checks move into webview_cost.py. .github/workflows/one-process.yml is deleted.
  - The Release workflow's probe step from astromech-v3u4 runs `webview_cost.py --checks-only`, or keeps a short probe; it doesn't keep two drivers.
  - `spikes/o1-one-process/` stays, as spikes do, with its row in spikes/README.md naming what it answered.
- [x] ARCHITECTURE §18.1's S11 text says memory is reported (astromech-01ft), and the rest of S11 is measured by this harness.
- [x] docs/diagrams/08-app-processes.mmd shows one app process and its speech worker. Its PNG is re-rendered, or, if the renderer can't run here, marked stale in §18.1 with a follow-up item.
- [x] A last search of docs/ and docs/guide for `astromech-shell` and "background process" finds only history and the forwarder's note. `python3 scripts/check-guide.py` passes.
- [ ] The App window cost workflow passes on the PR on Windows, and its job summary is linked in an evidence receipt on astromech-y6d6, for success criterion one-process.

## Scope

scripts/webview_cost.py, scripts/test_webview_cost.py, .github/workflows/webview-cost.yml, the probe and the spike workflow, docs/ARCHITECTURE.md §18.1, docs/diagrams/08-app-processes.*, spikes/README.md.

## Context

- Today's harness: webview_cost.py:105-306.
- The spike's probe, and its reports on astromech-wgtr.
- The current Windows baseline: the core 6.2 MB and the window group 143 MB (run 42349317319).

## Notes

- 2026-11-07 [note] From spike astromech-wgtr (evidence astromech-t55z, astromech-mb4f; windows-latest, Server 2025, 4 CPUs): the baseline for the one app's cost. Private working set of the app plus its WebView2 processes, 30 s idle: spike windowless 3.3 to 3.5 MB (CPU 0.0%); window open 124.1 MB mean, 126.3 MB max, 7 processes (CPU 0.25 to 0.26% of the machine, about 1% of one core); after closes 1-5: 9.6, 9.7, 8.8, 7.9, 7.7 MB (run 2: 9.7 to 7.4 MB), one process and no WebView2 left, which was gone about 2.3 s after WM_CLOSE; the plain tray-icon baseline (tray.rs today, no Tauri) 1.9 MB. Today's core alone is 6.2 MB and the window group 143 MB (webview-cost run 42349317319). The spike has no pipeline, voice or caches, so the real app adds those. scripts/one_process_probe.py's measure() and its WebView2 ownership rule (descent, --webview-exe-name, the EBWebView folder) can be reused.
- 2026-11-07 [note] From finding astromech-k6f1 (wgtr review): when check 6 moves into webview_cost.py's gates, require the crashed window's close to pass, and report the tray fallback (icon built, no rectangle) as a degradation, not a pass.
- 2026-11-07 [decision] The tray fallback (icon built, as 'tray icon ready' logs, but ui_state.tray false) is a third verdict, DEGRADED: shown in bold in the report, but it doesn't fail the run; a tray that wasn't built, or no answer, fails
  Reason: Finding k6f1 asks for a degradation, not a pass; failing the run would make a runner with no notification area (Linux xvfb always) indistinguishable from a broken tray
  Rejected: fail the run on the fallback
- 2026-11-07 [decision] The probe's check 4 (open by a second start) and check 7 (quit with the window open: exit 0, no WebView2 left) are gates too, beside the five Done when lists; the windowless gate also requires exactly one app process in the profile (success criterion one-process). Checks 2 (Raw Input sink) and 3 (tray churn) need the spike's test hooks and are dropped; Raw Input is the onboarding smoke's kill-switch runs
  Reason: The Done when line folds the probe's other checks into webview_cost.py; dropping open and quit would lose checks Release ran
- 2026-11-07 [decision] Cold start's windowless samples are the five window opens of the open-and-close cycles in one app process (the open plus four reopens), not five fresh --background processes as before
  Reason: The cycles already reopen the window by a second start, which is the measured path; separate processes would add five more app starts and minutes to the job
  Rejected: a fresh --background process per sample
- 2026-11-07 [decision] Release's --checks-only runs windowless, open, the five closes, both crashes and quit, and skips the page changes, the 30 s waits after closes and both cold starts; it reports memory and CPU but doesn't judge CPU or cold start. It runs the installed exe in a scratch profile, like App window cost
  Reason: Release's runner gates the process checks; cost is App window cost's, and gating it twice on different runners only adds noise
- 2026-11-07 [decision] On Linux the closes and crashes don't run (no WM_CLOSE under xvfb without a window manager; WebView2-specific crash kinds) and their gates show 'not run', which doesn't fail; on Windows a missing close or crash fails
  Reason: Linux reports only (continue-on-error); a not-run gate is honest where a pass would not be
- 2026-11-07 [decision] Restart mode (finding bnrg) runs on no CI workflow after one-process.yml goes: astromech-fmub runs it by hand at Sumeet's PC against the spike, and astromech-dl53's job is to run it against the installed app; the probe's docstring says to run it by hand on Windows after changing it
  Reason: The Done when deletes the spike workflow, and dl53 (blocked on decision lkwj) owns the job; a restart job here would duplicate dl53's
- 2026-11-07 [resolution] Findings zocz, 9nze and m6f9 fixed: the close gate requires each reopen to exit 0 and load a page (reopen_ok); quit requires ui_state window open and WebView2 present just before it (window_before, webview_before); cold start requires 5 samples where the closes run (Windows); a tray that had a rectangle and later reads fallback fails, and only fallback at every reading is DEGRADED. Tests for each (28 now), each seen to fail with its check removed. Stale pointers: the spike's main.rs header, astromech-mqqa's line, and a note on astromech-v3u4 that webview_cost.py --checks-only replaced the probe step

## Report

Implementer report (agent-factory implement). Committed on astromech-y6d6; not yet run on Windows CI.

Outcome against Done when: lines 2, 4, 5, 6 and 7 hold and are ticked; lines 1, 3 and 8 wait on Windows CI (App window cost on PR #218). `scripts/webview_cost.py` now runs the one app in a scratch profile windowless (`--background`), open (a second start with `--route keybinds`, idle, then page changes), closed again over 5 WM_CLOSE cycles (WebView2 gone time, the app's private working set 30 s after each close), through a renderer and a browser-process kill, and to quit, plus cold start (the median of the five window opens in the running app, and with the app not running). WebView2 ownership is the probe's rule (`webview_owner`: descent, `--webview-exe-name`, the `EBWebView` folder), replacing `tree()`. `judge` gates: no WebView2 and one app process windowless; the open; page changes; no WebView2 within 15 s of each close; the app and its control API alive after each close and crash; each crashed window recovered in place and its close passing (finding k6f1); quit; cold start < 2 s; idle CPU <= 1% windowless and open. A tray built with no rectangle at every reading is DEGRADED, not a pass (k6f1); one that had a rectangle and loses it fails (m6f9). Each cycle's reopen must exit 0 and load, and quit must start from the window open (zocz). Memory rows are `reported`, against the spike's baseline, and `IDLE_MB_MAX` is gone. `scripts/one_process_probe.py` keeps only `--restart` (380 lines, from 929); `.github/workflows/one-process.yml` is deleted; Release's probe step runs `webview_cost.py --checks-only` on the installed app. webview-cost.yml's header describes the one process, and its paths add the harness test and app.rs, tray.rs and cli.rs. ARCHITECTURE §18.1 (and §14's S11 row) say memory is reported (01ft) and what the harness measures; the 08-app-processes diagram shows one app process, its WebView2 while the window is open and the speech worker, re-rendered.

Proof (Linux, after the last change):
- `python3 scripts/test_webview_cost.py`: 28 tests OK (24, then 4 more for the review's findings zocz and m6f9: each reopen must load, quit needs the window open, 5 cold-start samples on Windows, a tray that loses its rectangle fails), covering judge and the report for passing and failing runs, including a WebView2 process left after a close (`test_a_webview2_process_left_after_a_close_fails`), the crashed window's close (`test_a_crashed_windows_close_must_pass`), the tray fallback (`test_the_tray_fallback_is_degraded_not_a_pass`), memory with no pass mark, `--checks-only` and Linux's not-run rows, and the ownership rule (an orphaned renderer counts). Four mutations of webview_cost.py (the 15 s bound, the tray verdict, the crash close, the windowless WebView2 check) each made the suite fail.
- py_compile and pyflakes (offline uv cache) clean on webview_cost.py, its test and one_process_probe.py; the probe refuses to run without `--restart`.
- actionlint 1.7.12 on webview-cost.yml and release.yml: clean; both parse as YAML.
- A live `--checks-only` run against the headless debug build (no `window` feature, so no window or tray), with HOME and the profile in scratch: launch, the control API, `ui_state`, sampling, quit, the log copy and the report all ran; windowless and quit PASS, open and tray FAIL as expected without a window, the Windows-only rows `not run`; no process was left.
- `python3 scripts/check-guide.py`: 22 pages, 0 problems. A search of docs/ for `astromech-shell` and "background process" finds only ARCHITECTURE's v0.x history, V1-DESIGN's forwarder notes and the diagram's forwarder label.
- The diagram was rendered offline with mermaid 11.17.2 (a copy already on this host) in headless Chrome, at 2x, with docs/diagrams/mermaid-config.json; the image was checked by eye.
- Not run here: the harness against a window build (no WebKitGTK dev files on this host) and anything on Windows.

### Deviations

- Beyond the listed gates: open, quit and one app process windowless, from the probe's checks 4 and 7 (decision note). Checks 2 and 3 need the spike's hooks and are dropped.
- Cold start's five windowless samples come from the cycles' reopens in one process (decision note).
- On Linux the closes and crashes don't run and show `not run` (decision note).
- Also changed, outside Scope: CONTRIBUTING (the Release step's description and an App window cost bullet), ARCHITECTURE §14's S11 exit criterion (it still said < 250 MB), and three comments in app.rs and host.rs that named the probe.

### Open items

- Lines 1, 3 and 8: App window cost on PR #218 (Windows gates, Linux reports), then the evidence receipt on astromech-y6d6. The Release build also runs, since release.yml changed: its `one-process checks (installed app)` step.
- Unverified until Windows CI: whether Tauri's EBWebView follows the scratch LOCALAPPDATA (both folders are matched, and `--webview-exe-name` covers it either way); the renderer and browser `--type` matching under the new ownership code; the 30 s after-close memory reading.
- Restart mode runs on no workflow now (finding bnrg; decision note): astromech-fmub by hand and astromech-dl53's job.
- Independent review is outstanding.
