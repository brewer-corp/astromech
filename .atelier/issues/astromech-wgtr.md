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
status: "todo"
title: "Spike: a windowless Tauri tray app on Windows CI (no window, window crash, close, Raw Input, restart registration)"
updated_at: "2026-11-07T11:57:35.952505855+00:00"
---

## Goal

Before any migration, find out on Windows whether a Tauri app can:
- hold astromech's tray with no window;
- survive the window crashing;
- free WebView2 completely when the window is closed;
- leave astromech's own Raw Input listener receiving keys.

Also measure what the windowless process costs, and how far Windows' restart registration covers a crash. Every later child depends on these answers. This host is Linux, so the proof runs on Windows CI.

## Done when

- [ ] `spikes/o1-one-process/` is a standalone Cargo project, outside the workspace like the other spikes (Cargo.toml `exclude`). It is a minimal Tauri app pinned to the workspace's `tauri` version (2.12.3 in Cargo.lock), with the `tray-icon` and `custom-protocol` features and one static page.
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
- [ ] `scripts/one_process_probe.py --app <exe> --endpoint <core.json> --out <dir>` drives any app with that interface (the spike now, the real app later).
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
- [ ] `.github/workflows/one-process.yml` (name "One process (Windows)") runs on `pull_request` paths `spikes/o1-one-process/**`, `scripts/one_process_probe.py` and itself, and on `workflow_dispatch`. It has two jobs on `windows-latest`:
  - **`probe`** builds the spike in release mode and runs the probe. The checks that answer decision astromech-6i8s's premises must pass. It reports, without gating (decision astromech-01ft), the private working set and CPU over 30 s idle for `--background` and for `--plain-tray`, the same with the window open, and after each close.
  - **`restart`** reports, without gating, whether Windows restarts the app after each crash kind. The app registers restart with `RegisterApplicationRestart("--background --restarted", RESTART_NO_PATCH | RESTART_NO_REBOOT)`.
    - Each kind is a fresh process, started with the crash hook on and left up for 65 s: `abort`, `access_violation` and `main_thread_panic`. It also checks a process ended with TerminateProcess (as `taskkill /F` does) and an `abort` at 10 s.
    - A restart counts when a process with `--restarted` on its command line appears within 90 s.
    - It first records the state of the WerSvc service and the `Windows Error Reporting` registry values (`Disabled`, `DontShowUI`), so "no restart" can be told apart from "WER is off on the runner".
- [ ] Both job reports and the workflow run's link are in an evidence receipt on this issue. The Close Reason answers each check, and says what astromech-v3u4 must add: a `ProcessFailed` handler, starting the shared input source after `build`, or `abort` for uncaught panics on the main thread.
- [ ] **Stop rule.** A failing check is the spike's answer, not something to work around. Record it in the receipt.
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
