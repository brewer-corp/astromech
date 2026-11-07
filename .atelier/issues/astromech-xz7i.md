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
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "webview_cost.py measures the one app windowless, open and closed again, and fails on a WebView2 process left behind"
updated_at: "2026-11-07T11:57:35.951210811+00:00"
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
- [ ] Its `judge` gates:
  - no WebView2 process windowless;
  - none within 15 s of each close;
  - the app alive after each crash and close;
  - cold start under 2 s;
  - idle CPU at most 1% of the machine, both windowless and open.

  Memory rows are reported with no pass mark, and `IDLE_MB_MAX` goes.

  scripts/test_webview_cost.py covers `judge` and the report for passing and failing runs, including a WebView2 process left after a close.
- [ ] webview-cost.yml builds the one app and runs the harness: Windows gates and Linux reports, as now. It runs on PRs that touch app/src-tauri/**, the app, crates/astromech-core/src/{app,tray,cli}.rs or the harness. Its header comment describes the one process.
- [ ] scripts/one_process_probe.py is reduced to its restart mode, which astromech-dl53's job uses, and its other checks move into webview_cost.py. .github/workflows/one-process.yml is deleted.
  - The Release workflow's probe step from astromech-v3u4 runs `webview_cost.py --checks-only`, or keeps a short probe; it doesn't keep two drivers.
  - `spikes/o1-one-process/` stays, as spikes do, with its row in spikes/README.md naming what it answered.
- [ ] ARCHITECTURE §18.1's S11 text says memory is reported (astromech-01ft), and the rest of S11 is measured by this harness.
- [ ] docs/diagrams/08-app-processes.mmd shows one app process and its speech worker. Its PNG is re-rendered, or, if the renderer can't run here, marked stale in §18.1 with a follow-up item.
- [ ] A last search of docs/ and docs/guide for `astromech-shell` and "background process" finds only history and the forwarder's note. `python3 scripts/check-guide.py` passes.
- [ ] The App window cost workflow passes on the PR on Windows, and its job summary is linked in an evidence receipt on astromech-y6d6, for success criterion one-process.

## Scope

scripts/webview_cost.py, scripts/test_webview_cost.py, .github/workflows/webview-cost.yml, the probe and the spike workflow, docs/ARCHITECTURE.md §18.1, docs/diagrams/08-app-processes.*, spikes/README.md.

## Context

- Today's harness: webview_cost.py:105-306.
- The spike's probe, and its reports on astromech-wgtr.
- The current Windows baseline: the core 6.2 MB and the window group 143 MB (run 42349317319).
