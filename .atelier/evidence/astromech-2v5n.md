---
created_at: "2026-11-07T22:29:42.909264475+00:00"
id: "astromech-2v5n"
evidence_type: "validation"
commit: "376cb82478640cc285df0c314a0cccfa856f51a1"
result: "pass"
captured_at: "2026-11-07T22:29:42.909202940+00:00"
uri: "https://github.com/brewer-corp/astromech/actions/runs/42419543386"
agent_identity: "astromech-xz7i implementer (Claude)"
target:
  kind: "issue"
  id: "astromech-xz7i"
  role: "validates"
criteria:
- slug: "one-process"
  result: "pass"
criteria_mission: "astromech-kjfs"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xz7i"
    role: "validates"
  - kind: "issue"
    id: "astromech-y6d6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "PR #218 at 55668f02 (reports name GITHUB_SHA c84de377, the PR's merge commit of 55668f02), all 9 checks passed. App window cost run 42419543386: job 'WebView2 (Windows)' 128544854646 (windows-latest, Server 2025, 4 CPUs), harness tests 'Ran 28 tests ... OK', scripts/webview_cost.py on target/release/astromech-core.exe exit 0, artifact webview-cost-windows. Gates, all PASS: windowless (--background) 1 app process, WebView2 seen []; open by a second start --route keybinds exit 0, page load 0.616 s, 6 WebView2 processes (browser, renderer, gpu-process, crashpad-handler, 2 utility; each matched by descent and the EBWebView folder, all but crashpad also by --webview-exe-name); page changes 8 asked, 8 received, 0 extra windows, exit codes [0]; close, 5 cycles: reopens exit 0 at 0.598, 0.58, 0.598, 0.584 s, WebView2 gone 2.25, 1.09, 1.09, 1.09, 1.09 s after WM_CLOSE (window closed 0.04-0.06 s), none left, 1 app process after each; app and control API alive after closes 1-5 and both crashes; window crash: renderer killed (ProcessFailed kind 1, 'window: reloading the page'), browser killed (kind 0, 'window: rebuilt at keybinds'), each recovered in place and its close freed WebView2 in 1.09 s; quit from the window open with 6 WebView2 processes: exit code 0, none left; cold start median 0.6 s over 5 window opens in the running app [0.616, 0.598, 0.58, 0.598, 0.584], 0.626 s with the app not running (new profile 3.17 s, not judged); idle CPU windowless 0.37% of the machine, open 0.45% (in use 0.57%); tray rect at all 8 readings. Memory, reported with no pass mark (decision astromech-01ft), private working set: windowless 7.2 MB mean, 7.8 max, 1 process (spike 3.3-3.5); open idle on keybinds 150.3 MB mean, 151.6 max, 7 processes, in use peak 151.8 (spike 124.1 mean); 30 s after closes 1-5: 13.1, 11.5, 11.3, 11.4, 10.8 MB, 1 process each (spike 9.6 to 7.7). The gone times are polling-bound: each scan of the process table takes about 0.5 s on the runner, so 1.09 s is an upper bound. Job 'WebKitGTK (Linux)' 128544854517 (reports only): windowless, open, page changes (15 of 15), quit, cold start (0.76 / 0.833 s) and idle CPU (0.28 / 0.17%) PASS; closes and crashes not run (Windows only); tray DEGRADED (fallback, no notification area under xvfb); memory windowless 43.4 MB, open 250.1 MB. Release build run 42419543337, job 'Windows installer (build, install, smoke test)' 128544822261, step 'one-process checks (installed app)': webview_cost.py --checks-only on the installed astromech-core.exe passed every check gate (windowless 1 process and no WebView2; open 0.398 s; 5 closes with reopens loading, WebView2 gone in 1.05-1.06 s; renderer and browser crash recovered and closed; quit; tray rect x8), artifact one-process-app-checks. The app logs show only the two expected ProcessFailed lines. Success criterion one-process: one app process plus its speech worker, and no WebView2 process left with the window closed."
updated_at: "2026-11-07T22:29:51.146449305+00:00"
---

PR #218 at 55668f02 (reports name GITHUB_SHA c84de377, the PR's merge commit of 55668f02), all 9 checks passed. App window cost run 42419543386: job 'WebView2 (Windows)' 128544854646 (windows-latest, Server 2025, 4 CPUs), harness tests 'Ran 28 tests ... OK', scripts/webview_cost.py on target/release/astromech-core.exe exit 0, artifact webview-cost-windows. Gates, all PASS: windowless (--background) 1 app process, WebView2 seen []; open by a second start --route keybinds exit 0, page load 0.616 s, 6 WebView2 processes (browser, renderer, gpu-process, crashpad-handler, 2 utility; each matched by descent and the EBWebView folder, all but crashpad also by --webview-exe-name); page changes 8 asked, 8 received, 0 extra windows, exit codes [0]; close, 5 cycles: reopens exit 0 at 0.598, 0.58, 0.598, 0.584 s, WebView2 gone 2.25, 1.09, 1.09, 1.09, 1.09 s after WM_CLOSE (window closed 0.04-0.06 s), none left, 1 app process after each; app and control API alive after closes 1-5 and both crashes; window crash: renderer killed (ProcessFailed kind 1, 'window: reloading the page'), browser killed (kind 0, 'window: rebuilt at keybinds'), each recovered in place and its close freed WebView2 in 1.09 s; quit from the window open with 6 WebView2 processes: exit code 0, none left; cold start median 0.6 s over 5 window opens in the running app [0.616, 0.598, 0.58, 0.598, 0.584], 0.626 s with the app not running (new profile 3.17 s, not judged); idle CPU windowless 0.37% of the machine, open 0.45% (in use 0.57%); tray rect at all 8 readings. Memory, reported with no pass mark (decision astromech-01ft), private working set: windowless 7.2 MB mean, 7.8 max, 1 process (spike 3.3-3.5); open idle on keybinds 150.3 MB mean, 151.6 max, 7 processes, in use peak 151.8 (spike 124.1 mean); 30 s after closes 1-5: 13.1, 11.5, 11.3, 11.4, 10.8 MB, 1 process each (spike 9.6 to 7.7). The gone times are polling-bound: each scan of the process table takes about 0.5 s on the runner, so 1.09 s is an upper bound. Job 'WebKitGTK (Linux)' 128544854517 (reports only): windowless, open, page changes (15 of 15), quit, cold start (0.76 / 0.833 s) and idle CPU (0.28 / 0.17%) PASS; closes and crashes not run (Windows only); tray DEGRADED (fallback, no notification area under xvfb); memory windowless 43.4 MB, open 250.1 MB. Release build run 42419543337, job 'Windows installer (build, install, smoke test)' 128544822261, step 'one-process checks (installed app)': webview_cost.py --checks-only on the installed astromech-core.exe passed every check gate (windowless 1 process and no WebView2; open 0.398 s; 5 closes with reopens loading, WebView2 gone in 1.05-1.06 s; renderer and browser crash recovered and closed; quit; tray rect x8), artifact one-process-app-checks. The app logs show only the two expected ProcessFailed lines. Success criterion one-process: one app process plus its speech worker, and no WebView2 process left with the window closed.
