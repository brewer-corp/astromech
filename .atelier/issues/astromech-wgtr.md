---
created_at: "2026-11-07T11:57:35.952505855+00:00"
id: "astromech-wgtr"
issue_type: "spike"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T14:01:30.453166519+00:00"
status: "done"
title: "Spike: a windowless Tauri tray app on Windows CI (no window, window crash, close, Raw Input, restart registration)"
updated_at: "2026-11-07T14:01:30.453166519+00:00"
---

## Goal

Before any migration, find out on Windows whether a Tauri app can:
- hold astromech's tray with no window;
- survive the window crashing;
- free WebView2 completely when the window is closed;
- leave astromech's own Raw Input listener receiving keys.

Also measure what the windowless process costs, and how far Windows' restart registration covers a crash. Every later child depends on these answers. This host is Linux, so the proof runs on Windows CI.

## Done when

- [x] `spikes/o1-one-process/` is a standalone Cargo project, outside the workspace like the other spikes (Cargo.toml `exclude`). It is a minimal Tauri app pinned to the workspace's `tauri` version (2.12.3 in Cargo.lock), with the `tray-icon` and `custom-protocol` features and one static page.
  - It has `"windows": []`. A tray icon is built with a menu shaped like crates/astromech-core/src/tray.rs: a status line, Open, a Pause check item, an optional line, and Quit.
  - Dark menus (tray.rs:35-54) are applied after `Builder::build`, because tao calls `allow_dark_mode_for_app(true)` when it builds its event loop (tao-0.37.1 platform_impl/windows/event_loop.rs:190).
  - `RunEvent::ExitRequested` with no exit code is prevented, so closing the last window doesn't end the process.
  - The page-load log line has the shell's format: `window: page loaded N ms after start` (app/src-tauri/src/window.rs:197-206).
  - It starts astromech's own listener, `astromech_input::listen::win::rawinput::start` (a path dependency on crates/astromech-input), with a `--sink-before-build` or `--sink-after-build` option. The spike counts the keys that listener sees.
  - Its control port uses the core's wire: one JSON line `{token, cmd, args?}` in, `{"ok": true, "value": V}` or `{"ok": false, "error": E}` back, with the endpoint in `%LOCALAPPDATA%\astromech-spike\core.json`. Commands:
    - `ui_state`: `{window: "open"|"closed", tray: bool, sink_keys: N}`, where `tray` is `TrayIcon::rect()` read on the main thread;
    - `open_window` with a route;
    - `tray_churn`: from a worker thread, rebuild the menu 50 times and set the text, enabled and checked states, as tray.rs:133-151 does;
    - `quit`, sent from a worker thread;
    - `crash`, with kind `abort`, `access_violation` or `main_thread_panic`.
  - `--background` starts it with no window. `--route X`, as a second start, hands over through a named mutex and the control port. `--plain-tray` runs the same tray with the `tray-icon` crate and a Win32 pump, as tray.rs does today, without Tauri, as a baseline.
- [x] `scripts/one_process_probe.py --app <exe> --endpoint <core.json> --out <dir>` drives any app with that interface (the spike now, the real app later).
  - A **WebView2 process belongs to the app** when it is a descendant of the app, or its command line has `--webview-exe-name=<the app's exe name>`, or its command line names the app's `EBWebView` user-data folder. A renderer orphaned by a dead parent still counts.
  - It asserts:
    1. **Windowless.** After `--background`, `ui_state.tray` is true within 10 s and the window is closed. For 30 s no WebView2 process belongs to the app.
    2. **Raw Input survives Tauri.** With `--sink-after-build`, an F-key sent by SendInput raises `sink_keys`: at start, after the window opens, after it closes, and after `tray_churn`. With `--sink-before-build`, the same is recorded as the answer for the case where a session's listener starts before the event loop is built. tao registers keyboard and mouse to its own hidden window when it builds the loop (tao raw_input.rs:128-157, called at event_loop.rs:200). Tauri sets the device filter again in `Builder::build` (tauri-2.12.3 app.rs:2534). Windows keeps the last registration for each device class in a process.
    3. **Tray churn.** `tray_churn` answers ok, and `ui_state` still answers afterwards.
    4. **Open.** A second start with `--route keybinds` exits 0, the page-load line appears within 10 s, and WebView2 processes belong to the app.
    5. **Close.** `WM_CLOSE` is posted to the window, found by its title as a click on X would be. Within 10 s the window is closed, within 15 s no WebView2 process belongs to the app, and the app and tray are still there. This is repeated over 5 open and close cycles.
    6. **Window crash.**
       - With the window open, the app's WebView2 renderer (`--type=renderer`) is killed, then, after a reopen, its browser process (no `--type`).
       - After each kill, the app is alive, `ui_state.tray` is true and the control port answers.
       - Closing and reopening loads the page again.
       - The report says what the dead window showed, and whether the app needed its own `ProcessFailed` handler.
    7. **Quit.** `quit` from a worker thread ends the process with exit code 0 within 5 s, with no WebView2 process belonging to the app.
- [x] `.github/workflows/one-process.yml` (name "One process (Windows)") runs on `pull_request` paths `spikes/o1-one-process/**`, `scripts/one_process_probe.py` and itself, and on `workflow_dispatch`. It has two jobs on `windows-latest`:
  - **`probe`** builds the spike in release mode and runs the probe. The checks that answer decision astromech-6i8s's premises must pass. It reports, without gating (decision astromech-01ft), the private working set and CPU over 30 s idle for `--background` and for `--plain-tray`, the same with the window open, and after each close.
  - **`restart`** reports, without gating, whether Windows restarts the app after each crash kind. The app registers restart with `RegisterApplicationRestart("--background --restarted", RESTART_NO_PATCH | RESTART_NO_REBOOT)`.
    - Each kind is a fresh process, started with the crash hook on and left up for 65 s: `abort`, `access_violation` and `main_thread_panic`. It also checks a process ended with TerminateProcess (as `taskkill /F` does) and an `abort` at 10 s.
    - A restart counts when a process with `--restarted` on its command line appears within 90 s.
    - It first records the state of the WerSvc service and the `Windows Error Reporting` registry values (`Disabled`, `DontShowUI`), so "no restart" can be told apart from "WER is off on the runner".
- [x] Both job reports and the workflow run's link are in an evidence receipt on this issue. The Close Reason answers each check, and says what astromech-v3u4 must add: a `ProcessFailed` handler, starting the shared input source after `build`, or `abort` for uncaught panics on the main thread.
- [x] **Stop rule.** A failing check is the spike's answer, not something to work around. Record it in the receipt.
  - If a premise of decision astromech-6i8s fails, stop and take it to Sumeet before astromech-7i2k starts. The premises are: the tray works with no window; the app survives a window crash; closing the window ends WebView2; and an app-owned Raw Input listener can keep receiving keys.
  - If restart registration doesn't restart `abort` or access-violation crashes, and WER is on, stop and ask Sumeet. The strategy's Shape allows a watchdog, but success criterion one-process (one app process plus its speech worker) forbids a third process.

## Scope

Only `spikes/o1-one-process/`, `scripts/one_process_probe.py`, `.github/workflows/one-process.yml`, and a row in spikes/README.md. Nothing in crates/, app/ or the shipped app changes. This child doesn't need astromech-bp23 (PR #216). Every later child waits for bp23 to merge, and the branch is rebased onto main before astromech-7i2k starts.

## Context

- The Unknowns of astromech-y6d6, and the Risks of docs/strategies/one-tested-process.md.
- WebView2 is created only when wry builds a webview, so a windowless Tauri app should start no WebView2 process. That is checked here, not assumed.
- Today's baseline on windows-latest is 6.2 MB private working set for the core alone and 143 MB for the window group (webview-cost run 42349317319, commit bb7c25d7).
- tray.rs's own pump wakes every 100 ms (tray.rs:153-154, 202-214). tao's loop waits.
- Tauri 2.12.3 already depends on `tray-icon` 0.25.1 and `muda` 0.20 (Cargo.lock). The core uses `tray-icon` 0.26.

## Unknowns

- Whether the notification area exists in the runner's session, so that `TrayIcon::rect()` returns a rectangle there. If it doesn't, the tray part of check 1 falls back to the main-thread round trip of `ui_state` and the icon's creation result. Clicking the icon stays an after-merge check at Sumeet's PC, in astromech-v3u4.

## Report

Implementer report (spike, evidence only). Commits a54413ea, 388ab5a7 and a2686ace on astromech-y6d6; Windows CI runs 42374255964 and 42376155384 on draft PR #218. Evidence: astromech-t55z (probe, run 1), astromech-mb4f (probe, run 2), astromech-fqw9 (restart table, run 2).

Outcome: every premise of decision astromech-6i8s held on Windows CI, and the stop rule doesn't stop the migration. The tray works with no window; the app survives a window crash; closing the window ends WebView2; an app-owned Raw Input listener keeps receiving keys when it starts after Builder::build. Restart registration couldn't be shown on CI, because WER never engaged on the runner. That makes it inconclusive, not a failed premise, and it moves to an after-merge check at Sumeet's PC (astromech-dl53). Checks 1-7 passed in both runs.

Answers, check by check:
1. Windowless: with "windows": [] and --background, TrayIcon::rect() had a rectangle within 10 s (the runner has a notification area, so the fallback wasn't needed), the window was closed, and no WebView2 process belonged to the app in 30 s. WebView2 starts only when a webview is built.
2. Raw Input: with astromech's listener (astromech_input::listen::win::rawinput::start) started after Builder::build, an F13 sent by SendInput was seen at start, after the window opened, after each of 5 closes and after tray churn (8 of 8). Started before build, it saw nothing at any point: tao's event loop registers keyboard and mouse to its hidden window, and the last registration per device class wins. So the shared source starts after build (astromech-7i2k).
3. Tray churn: 50 menu rebuilds with text, enabled and checked changes from a worker thread took 91-96 ms; ui_state answered after. Tauri runs menu calls on the main thread.
4. Open: a second start with --route keybinds handed over through the named mutex and control port, exited 0, and the page loaded about 3.1 s after the second start; 6 WebView2 processes belonged to the app.
5. Close: WM_CLOSE by title closed the window in about 0.24 s and every WebView2 process was gone in about 2.3 s, in 5 of 5 cycles; the app and tray stayed (RunEvent::ExitRequested with no code is prevented).
6. Window crash: killing the renderer left WebView2's "This page is having a problem" page (Error code 15) with the browser still up; killing the browser process left a blank white window and no WebView2. Both times the app, tray and control port lived, ProcessFailed fired (kind 1, then kind 0), and close plus reopen loaded the page. The app doesn't need a ProcessFailed handler to survive, but needs one to recover the window in place: reload after a renderer exit, rebuild the webview after a browser exit.
7. Quit: app.exit(0) from a worker thread exited 0 in 0.24 s with the window open, with no WebView2 left.

Cost (reported, not gated, decision astromech-01ft), private working set of the app plus its WebView2: windowless 3.3-3.5 MB and 0.0% CPU; window open 124.1 MB and 0.25% of the machine; after closes 9.7 falling to 7.4 MB with no WebView2; plain tray-icon baseline 1.9 MB. Today's core alone is 6.2 MB.

Restart registration (reported, not gated): RegisterApplicationRestart("--background --restarted", RESTART_NO_PATCH | RESTART_NO_REBOOT) returned 0. abort (0xC0000409), access_violation (0xC0000005), main_thread_panic (0x65), TerminateProcess and an abort at 10 s were tried in three configurations: the runner's WER, WER on as a desktop has it, and WER on with a default-error-mode launch. Nothing was restarted, no WER report was written, and no WerFault or dialog was seen. The runner hands children error mode 0x3 (SEM_NOGPFAULTERRORBOX) and refuses to let them leave its job. With error mode 0, WER still didn't engage, for an unidentified reason (the runner's job or the Server SKU).

What astromech-v3u4 must add: a ProcessFailed handler (reload on kind 1, rebuild the webview on kind 0); starting the shared input source after build (astromech-7i2k); and abort for uncaught panics on the main thread, because a main-thread panic exits 0x65 through tao with no WER and so can never be restarted. Notes with the data are on astromech-7i2k, astromech-v3u4, astromech-dl53 and astromech-xz7i. astromech-dl53's Done when now has its CI job assert registration and a faked --restarted start, plus an after-merge needs-human restart check at Sumeet's PC whose failure takes the watchdog question to him.

Deviations: tauri-build is pinned to 2.7.2, the version in the workspace lock (2.12.3 is the tauri crate). --crash-hook both allows the crash command and registers for restart. The page beats once a second so ui_state can tell a live page from a dead one. The spike only logs ProcessFailed. The window's WebView2 data lives in %LOCALAPPDATA%\astromech-spike. Check 2 was extended from 4 to 8 points. The restart job runs a third configuration (WER on, clean launch) beyond the two asked for, and the spike logs its error mode and job limits.

Surprises: the runner's inherited error mode 0x3; WerSvc already running after the first configuration even though no report appeared; the app keeps 4-6 MB more after a close than before the first open, falling slowly.

Disposition: discard the spike's code once v3u4 lands (keep the learning, recorded here). Retain scripts/one_process_probe.py: its checks, its WebView2 ownership rule and its restart mode are meant to run against the real app (astromech-xz7i, astromech-dl53).

Open items: independent review of the three commits is outstanding. Restart is unproven until Sumeet's PC (astromech-dl53). The tray icon was never clicked, keys were injected (not a physical keyboard), and Elite wasn't running (astromech-v3u4's play check). WerFault was polled only after the crashed process exited.
