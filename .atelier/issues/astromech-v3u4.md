---
created_at: "2026-11-07T11:57:35.951474903+00:00"
id: "astromech-v3u4"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7149"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-nade"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "The one app: astromech-core.exe runs the core, the tray and the window in one Tauri process"
updated_at: "2026-11-07T22:12:15.565189035+00:00"
---

## Goal

`astromech-core.exe` is the Tauri app. One process holds the tray, the pipeline, voice, input and, when asked, the window (decision astromech-6i8s; decision astromech-nade). The shell process, its watchdog and the start-the-core plumbing are gone.

**In-game behaviour (tray, kill switch, voice pause, quit):** all of these move to Tauri's event loop and must behave exactly as they do now.

## Done when

- [x] One binary, as decision astromech-nade says.
  - app/src-tauri's package builds the `astromech-core` binary. Its `main` is the core's command line, moved into the library (`astromech_core::cli`) with every option unchanged; `--webview-debug-port` is added from window.rs:154-157. Console mode and `--apply-update` still run before any Tauri setup (main.rs:144-184).
  - The Tauri host is built on astromech-vucf's headless assembly function, the one that builds the core from its boundaries and a `Paths`. The host adds the tray and the window around it, instead of reshaping `app::run` (app.rs:1490-1673) a second time. Only the Tauri host, not the headless build, gets a tray and a window.
  - crates/astromech-core keeps only its `sign-update` binary.
  - Without the `window` feature the binary runs headless (app.rs:1685-1698), so `cargo build --workspace` still needs no webkit2gtk. The headless binary serves scripts/e2e-gear-down.sh, scripts/mvp_criteria.py and CI's gear-down step, with their build commands updated.
  - scripts/check-deps.py and ARCHITECTURE §4 name the package.
- [x] The tray runs on Tauri's tray API (the `tray-icon` feature), and the core's direct `tray-icon` 0.26 dependency is dropped.
  - It has the same items, texts, conditional lines, left-click and dark menus as tray.rs.
  - A pure function from the menu's state (tray.rs `Shape` and its texts, tray.rs:56-86, 102-122) to the list of items is unit-tested for each shape, so the port keeps the menu exact.
  - Dark menus (tray.rs:35-54) are applied after `Builder::build`, because tao calls `allow_dark_mode_for_app(true)` when it builds its event loop (tao-0.37.1 event_loop.rs:190).
  - `Core::notify` schedules a refresh on the main thread without waiting (`run_on_main_thread`), replacing `Ui::Refresh`.
  - No Tauri menu or window call is made while a `Core` lock is held. tray.rs:137 takes the session lock on the UI thread; that read happens before any menu call.
  - Quit, from the tray, the `quit` command or `install_update` (app.rs:540-544), calls `app.exit(0)`. The shutdown that ends `run` today, `deactivate("quit")` then `apply_on_quit` (app.rs:1669-1672), runs in `RunEvent::Exit`, so held keys are still released.
  - If the tray fails, the app logs it and runs on without it (app.rs:1675-1683).
- [x] **Input after `build`.** The shared input source from astromech-7i2k is started, or restarted with `restart()`, after `Builder::build`, before the session can attach. tao registers keyboard and mouse Raw Input to its own hidden window when it builds the loop (tao raw_input.rs:128-157, called at event_loop.rs:200; tauri-2.12.3 app.rs:2534 sets the filter again).
  - Opening and closing the window must not register again. If the spike's check 2 showed the window does, the source registers again after each window build.
  - The device-event filter is left at its default. `DeviceEventFilter::Always` would make tao call `RIDEV_REMOVE`, which unregisters astromech's own listener too.
- [x] The window.
  - `--background` (the Run key) creates no window and no WebView2 process. A start without it opens the window (app.rs:1664-1666).
  - `Core::open_window` builds the window, or focuses it and emits `route`. Callers are the tray, a second start through the mutex and `open_window` over the control API (app.rs:1498-1507), and an update's `--route settings` relaunch. This replaces spawning `SHELL_EXE` (app.rs:573-599).
  - Closing the window destroys it, and `ExitRequested` with no code is prevented.
  - Whatever the spike astromech-y6d6 found necessary is in, such as handling WebView2's `ProcessFailed` by destroying the dead window.
- [x] The window's `call` runs `Registry::dispatch` in process, on `spawn_blocking`, with `origin: AppWindow`. A string `args` becomes `text`, as `control::call_window` does (crates/astromech-api/src/control.rs:192-210), and progress goes to the call's channel. The window no longer reads `core.json`.
  - Removed: window.rs:33-79 (`core_alive`, `start_core`, `watchdog`), the `core-restarted` event (named in app/src/lib/transport.ts:5's comment) and its preview entry (app/src/preview/transport.ts:43), and `tauri-plugin-single-instance`.
  - app/src/lib/ui/staleTauri.browser.test.ts:45 is updated for the in-process error text.
  - `control::call_window` goes if nothing else calls it.
  - The press listener subscribes to the previous child's input source while the window exists, and unsubscribes when the window is destroyed.
- [x] A `local` command `ui_state` returns `{window, tray}` as in the spike, for the probes. The window's log lines, including `window: page loaded …`, go to `astromech-core.log`.
- [x] The window's copy no longer speaks of a separate background process:
  - app/src/modules/settings/View.svelte:112;
  - the version line in app/src/modules/diagnostics/View.svelte:46-48;
  - docs/guide/getting-started.md:70 and :127.
- [x] Packaging, so the release build and its smokes pass at this child:
  - tauri.conf.json: `mainBinaryName` is `astromech-core` and `externalBin` holds only the voice worker. scripts/stage-sidecars.sh stages only the worker.
  - .github/workflows/release.yml: the core is built by `tauri build` (`sign-update` and `mockgame` are still built separately). The smoke's file list drops `astromech-shell.exe`, and the pack-update list matches.
  - .github/workflows/ci.yml: the Windows job builds and runs clippy on the new package with `--features window`. ci.yml's gear-down step and .github/workflows/criteria.yml:61 build the headless binary from the new package.
  - scripts/onboarding_smoke.py starts `astromech-core.exe --webview-debug-port …` and finds the install by `astromech-core.exe`.
  - scripts/webview_cost.py and webview-cost.yml are retargeted to the one executable: the app is started with `--route` by a second start, and it logs to `astromech-core.log`. The "core not running" and "after a core crash" cold starts are replaced by "app not running" and "app running windowless". The fuller rework is astromech-xz7i.
- [x] **The kill switch in one process.** The onboarding smoke's kill-switch step from astromech-7i2k passes against the one app. That step taps F8 to kill and holds F8 for 1.2 s to re-arm, with the window open, in two runs: mock game after the app, and mock game before the app. Before the app is the case where the session's listener could start before Tauri's `build`. Each run then closes the window with `WM_CLOSE` and taps F8 once more, checking that the kill switch comes on.
- [x] `scripts/one_process_probe.py` from the spike passes against the installed app on Windows CI. It runs as a step of the Release workflow's installer job, with the endpoint `%LOCALAPPDATA%\astromech\data\core.json`. Its WebView2 match counts processes by command line as well as by descent. For the real app, check 2 (Raw Input) is covered by the onboarding smoke's kill-switch runs, so the probe skips it with `--no-sink`, and the app has no extra test hook.
- [x] Docs:
  - ARCHITECTURE §18.1 describes one process and links astromech-6i8s; §18.6 and the naming lines at §18 (1709-1712) are updated.
  - V1-DESIGN §7.1 and §7.10 describe the one process.
  - CONTRIBUTING's layout and build lines name the new package.
- [x] CONTRIBUTING's checks pass locally. On the PR, Windows CI, the Release build (installer smoke, onboarding smoke, probe) and App window cost pass.
- [ ] After merge (needs-human: Sumeet's PC, next to Elite, the S11 run). He checks:
  - At login, Start with Windows shows the tray and no window.
  - Left-clicking the tray icon opens the window. Pause voice and the kill switch work from the tray menu while Elite runs.
  - Push-to-talk and the kill switch work with the window open, and after it is closed.
  - Opening and closing the window in play causes no hitch he can see.
  - Quit releases everything.

  Stand-ins: the probe on Windows CI and the onboarding smoke's kill-switch check.

## Scope

app/src-tauri, crates/astromech-core (main.rs moves into the library; app.rs, tray.rs; instance.rs stays), crates/astromech-input (the source's `restart` call site only), crates/astromech-api (control.rs `call_window`), app/src (transport, the two views above, the preview transport), the scripts and workflows named above, and the docs named above.

Not in this child:
- crash restart (the next child);
- the upgrade from two-process installs and the forwarder (astromech-9lh3);
- the full cost harness (astromech-xz7i).

## Context

Spike results on astromech-wgtr (its Close Reason and evidence receipt). The Tauri `call` and window code to fold in is window.rs:84-94 (call), 96-144 (start_input), 162-234 (main and setup, page-load at 197-206).

## Notes

- 2026-11-07 [note] From spike astromech-wgtr (evidence astromech-t55z, astromech-mb4f): the app needs a WebView2 ProcessFailed handler to recover the window in place. It survives without one (the app, tray and control port lived, and close plus reopen loaded the page), but the window stays broken until the user closes it. A renderer exit (ProcessFailed kind 1) leaves WebView2's 'This page is having a problem' page (Error code 15) with the browser process still up: reload the webview. A browser-process exit (kind 0) leaves a blank white window and no WebView2 process: rebuild the webview (destroy and recreate the window at its route). The spike's observer is spikes/o1-one-process/src/app.rs observe_process_failed, via with_webview and webview2-com 0.39.1.
- 2026-11-07 [note] From spike astromech-wgtr (evidence astromech-fqw9): a panic on the main thread exits with code 0x65 (101), with no WER report. tao catches the panic in its callback and re-raises it after the event loop, so it's an ordinary exit and restart registration can never cover it. Abort on an uncaught main-thread panic (astromech-dl53's crash.rs line), so it goes through WER as a crash. Other spike answers for the one app: the tray works with "windows": [] (tray rect within 10 s), closing the last window must prevent RunEvent::ExitRequested with no code, dark menus go after Builder::build, a second start hands over in about 3 s, and quit from a worker thread (app.exit(0)) exits 0 in 0.24 s with no WebView2 left.
- 2026-11-07 [note] From finding astromech-3xyy (wgtr review): scripts/one_process_probe.py has no --no-sink option, check 3 calls the spike's tray_churn test hook, and main() fails unless checks 1-7 all ran, so as written it cannot pass against the installed app. Add --no-sink, a way to skip or serve check 3, and explicit --log, --user-data and --title for the installed app.
- 2026-11-07 [note] From finding astromech-yzyz (7i2k review): restart() returns Ok without registering while the first subscribe()'s start is still in progress (crates/astromech-input/src/listen/source.rs:99-104); if a session attached during Tauri's build, the restart would silently do nothing. Make restart() wait for an in-progress start, or make that order impossible here and test it.
- 2026-11-07 [decision] The input source registers after Builder::build because the core is booted after it (host::run: begin, build, boot); restart() is not called
  Reason: Nothing can subscribe before build: the session's listener starts from the core's game watch and the window's with the window, both after boot. That makes finding yzyz's order (restart during an in-progress start) impossible here. The onboarding smoke's second kill-switch run (mock game before the app) is the test: if the session's listener registered before tao's, F8 would never reach it
  Rejected: Boot the core before build and call restart() after it: needs restart() to wait for an in-progress start (yzyz), a change inside crates/astromech-input's source
- 2026-11-07 [decision] The core asks its UI through an app::Ui trait (refresh, open_window, window_open, state, quit), given as Boundaries::ui; the Tauri host's TauriUi and the headless app::Headless implement it. The Ui channel (Refresh, Quit) and Assembled.ui are gone
  Reason: notify must schedule a tray refresh on the main thread without waiting, and quit must end Tauri's loop; a trait at the boundary lets the core stay free of Tauri, the lib tests pass Headless, and run() waits on Headless
  Rejected: Keep the mpsc channel and a pump thread forwarding to Tauri: a second loop that only relays
- 2026-11-07 [decision] app::run is split into begin (logging, crash hook, mutex and hand-over), boot (autostart default, assemble, start, update thread), open_at_start and shutdown; run() is these headless, and host::run puts Tauri's build between begin and boot. Started also carries the Registry the window dispatches into
  Reason: The Done when line asks for the host on the assembly, not a second reshaping of run; the same four steps serve both, and the window must not open before the registry is managed in Tauri
- 2026-11-07 [decision] The tray's Install (Restart to update) and Save diagnostics clicks run on their own threads; Open, Pause, Kill switch, Check and Quit run in the click handler as before
  Reason: The main thread now also runs the window: an install drops the session (it waits for its threads) and diagnostics writes a zip, which would freeze the window meanwhile. What each item does is unchanged
- 2026-11-07 [decision] astromech_log::SHELL is removed and the window logs into astromech-core.log; Diagnostics' version line shows the one version (WINDOW_VERSION), with no window/background split
  Reason: One process writes one log and carries one build; the split line could only show a half-applied update of two exes
- 2026-11-07 [decision] The main-thread abort (crash.rs) is in this child: the hook aborts after logging and writing the marker when the panicking thread is named main
  Reason: The spike note on this child asks for it; astromech-dl53 keeps its tests of the catch_unwind recoveries and the WER restart
- 2026-11-07 [decision] one_process_probe.py: --no-sink skips check 2 and --no-churn skips check 3 (the expected set follows), and log lines without the spike's [pid prefix count for the app; webview_cost.py's later cold starts open the window in the app started --background, by a --route second start, plus one start with the app not running
  Reason: The app has no test hooks (Done when); its log has no pid prefix since one process writes it; the windowless app is how a player opens the window
- 2026-11-07 [result] MVP criteria failure on PR #218 at 0891652b (open_screen.left not_confirmed, 1 of 15): not a v3u4 regression. The mock wrote GuiFocus 2 to Status.json 727 ms after the key (keys.log), 1.27 s before the core's 2 s deadline, and the core never applied it; the next write came 1.9 s later. Status.json was read only on a file notification, with five 1 ms retries, so a lost notification or a failed read (half-written, or held: the mock's own logging thread stalled about 760 ms in the same second, splitting a keys.log line) lost the update until the next write. The console-mode pipeline v3u4 runs is unchanged code; b418f604's run had the same verify spread (max 949 ms vs 948 ms, the mock's 0-1000 ms delay), with keydown p50 7.0 vs 11.2 ms (a slower runner). Fix: an idle pass re-reads Status.json when its modified time or length differs from the last good read (pipeline.rs, tests a_status_write_without_a_notification_is_read_when_idle and the_pipeline_loop_reads_status_json_without_any_notification, both failing before); the mock writes each keys.log line in one write
- 2026-11-07 [decision] The window's open_window answers once the open is asked for: TauriUi hands every open (build or focus) to a window-open thread and returns; a failed build is logged there as before. Callers that need the window wait for ui_state or the page-load line (the onboarding smoke, upgrade_smoke and webview_cost already do; the probe bounds a second start's page load with --page-load-s, 30 s in Release)
  Reason: Building the window waits for WebView2, whose first start on a runner took 13 s, past the second start's 10 s control-API read timeout (spike run 42410580229). On main the core only spawned the shell process, so its reply never waited for the window either; for the user nothing changes but the 'already running, but it didn't answer' log line on a slow first start
  Rejected: Raise the second start's read timeout: still a guess at WebView2's worst start, and every caller waits for nothing it uses
- 2026-11-07 [note] Release's one-process probe step (the ticked line on scripts/one_process_probe.py in the Release installer job) was replaced in astromech-xz7i by scripts/webview_cost.py --checks-only on the installed app, which runs the same checks (windowless, open, five closes, the renderer and browser kills with recovery, quit); the probe keeps only --restart

## Report

Implementer report (agent-factory implement, with migrate's atomic change: every consumer is in this repo). Commits 26afef19 (the change), 0891652b (findings 2wu4, zo5n), 107e5c5d and f47c529b (the Status.json idle read, findings from the MVP criteria failure and 43sv) on astromech-y6d6. PR #218 passed every check at a2e6c32d (evidence astromech-ahj1).

Outcome: `astromech-core.exe` is built by app/src-tauri (package `astromech-app`, binary `astromech-core`). Its `main` is `astromech_core::cli::main` (the old main.rs, every option kept, plus `--webview-debug-port`) with the app it runs: `host::run` with the `window` feature, `app::run` (headless) without. `host::run` is `app::begin` (logging, crash hook, mutex and hand-over), Tauri's `Builder::build`, dark menus, `app::boot` (autostart default, `assemble`, `start`, update thread), the tray, `app::open_at_start`, and the loop; `RunEvent::Exit` runs `app::shutdown` (deactivate, apply_on_quit) then drops the control server and the mutex. The core reaches the tray and window through `app::Ui` (`Boundaries::ui`): `notify` posts one coalesced tray refresh with `run_on_main_thread`, `open_window` builds or focuses the window (off the main thread), quit is `app.exit(0)`, and `ui_state` (local) answers `{window, tray}`. The tray menu is `astromech_core::tray::{read, items}` (pure, unit-tested per shape) shown by app/src-tauri/src/tray.rs with Tauri's tray. The window's `call` dispatches `control::window_request` into the `Registry` (`Started.commands`) on `spawn_blocking`. A WebView2 ProcessFailed handler reloads after a renderer exit and rebuilds the window after a browser exit; the press listener subscribes per window and stops on `Destroyed`. Removed: window.rs (watchdog, core start, single-instance plugin, `core-restarted`), `SHELL_EXE`/`ASTROMECH_SHELL`, the `Ui` channel, the core's tray-icon/png deps and Win32 pump, `astromech_log::SHELL`. Packaging: `mainBinaryName` astromech-core, `externalBin` only the worker, release.yml/ci.yml/criteria.yml/webview-cost.yml retargeted, the probe runs as a Release installer-job step, the smoke's second kill-switch run and WM_CLOSE tap added.

Proof (Linux, after the last change):
- `cargo test --workspace --locked`: 1314 passed, 0 failed (134 binaries); tray::tests 5/5, and a reordered Install/Check fails `every_optional_line_in_its_place`; control::tests::the_windows_request_dispatched_in_process_is_the_same passes.
- `cargo clippy --workspace --all-targets --locked -D warnings` clean; `cargo clippy -p astromech-app --features window --all-targets --locked -D warnings` clean on Linux (fake .pc files) and for `--target x86_64-pc-windows-msvc` (fake cl, lib and llvm-rc; a planted type error in the Windows-only ProcessFailed code fails it), and the workspace's Windows-target clippy clean.
- `cargo fmt --all --check`, `scripts/check-deps.py` (32 crates), `scripts/check-guide.py`, `cargo deny check` pass; `pnpm lint`, `pnpm check`, `pnpm build`, `pnpm test` (534/534), `scripts/check-dist.py` pass; `scripts/test_webview_cost.py` 7/7; the three edited scripts compile.
- Headless binary live (temp HOME and LOCALAPPDATA): `--version` prints `astromech-core 0.0.0-dev`; `--background` publishes core.json with its pid; `status` ok; `ui_state` `{window: closed, tray: false}`, refused with `origin: app_window`; `open_window` answers the no-window error; a second start with `--route` exits 0 ("already running: asked it to open the window"); `quit` exits 0 and removes core.json.
- By inspection only on Linux: the Tauri tray, window, ProcessFailed and press listener at run time (no WebView2 here); Windows CI below runs them.

Proof (Windows CI, PR #218 at a2e6c32d, evidence astromech-ahj1), by Done when line:
- Input after build: Release run 42405888691's onboarding smoke, in its second run (mock game before the app, the session's listener at app start): 'kill switch: a tap of F8 killed, a 1.2 s hold re-armed, window open (game running before the app)'; after a window open and close, the WM_CLOSE tap still kills (both runs). The device-event filter is left alone (inspection).
- The window: the installed-app probe in the same job, 'PASS 1 windowless' (`--background`, no WebView2 for 30 s), 'PASS 4 open' (a second start opens it), 'PASS 5 close' (5 cycles: destroyed, WebView2 gone in about 2.1 s, app and tray alive), 'PASS 6 window crash' with recovered in place for both kills ('window: reloading the page' after kind 1, 'window: rebuilt at keybinds' after kind 0); the smoke's 'window opened by a second start (--route settings)'.
- Packaging: the Release job (tauri build of the app, installer smoke 'smoke test OK' without astromech-shell.exe, onboarding smoke and probe); CI run 42405888682's Windows job building and linting `astromech-app --features window` and the gear-down e2e on the headless build ('E2E OK'); MVP criteria (42405888696) on the headless release build; App window cost (42405888677) on the one app.
- The kill switch in one process: the onboarding smoke's six kill-switch lines, three per run (tap and hold with the window open, WM_CLOSE, tap with the window closed), 'Result: **PASS**'.
- The probe: checks 1 and 4-7 PASS with `--no-sink --no-churn --expect-recovery`; cost (reported) background 6.9 MB in 1 process, after each close 16.3 to 11.5 MB in 1 process, no WebView2 left.
- The checks line: the local checks above, and on the PR the Windows job (tests 1311 passed, 0 failed), the Release build (installer smoke, onboarding smoke, probe) and App window cost (WebView2: idle 149.2 MB, 0.37% CPU, cold start median 0.62 s windowless and 0.675 s not running).

### Deviations

- No `restart()` call: the core boots after `build`, so no subscriber can exist before it (decision note); the smoke's game-before-the-app run is the test.
- The smoke's first kill-switch run moved after the page tour and star map, since it now ends by closing the window.
- Tray Install and Save diagnostics run on threads (decision note).
- `call_window` stays: the every-window-command test calls it; it now wraps `window_request`, which the window dispatches.
- The main-thread abort is here (decision note), not only in dl53.
- V1-DESIGN §7.2, §7.3 and §7.7's file lists and single-instance/log lines were corrected too, since they named the removed exe and plugin; the 08-app-processes diagram is not redrawn (no Mermaid renderer here).

### Surprises

- Tauri's externalBin used to copy the staged core back over target/release/astromech-core.exe; with the core as the main binary it must not be in `externalBin` at all.
- The app's log lines have no `[pid` prefix, which the probe's per-pid filter assumed.

### Open items

- Not covered: the after-merge line, which is Sumeet's at his PC next to Elite (needs-human): Start with Windows at login, the tray's left-click and menu while Elite runs, push-to-talk and the kill switch with the window open and closed, no visible hitch on open and close, and Quit. The tray icon was never clicked on CI, and keys were injected, not pressed.
- Review: 26afef19 to 107e5c5d were reviewed (findings 2wu4, zo5n and 43sv; 107e5c5d approved in astromech-gcgf). f47c529b, the 43sv fix, ran green at a2e6c32d; whether it has had its own review is not recorded here.
- The 08-app-processes diagram, the upgrade path and the `astromech-shell.exe` forwarder (astromech-9lh3), crash restart (astromech-dl53) and the fuller cost harness (astromech-xz7i) are not here. spikes/o1-one-process stays until fmub and dl53 no longer need it.
