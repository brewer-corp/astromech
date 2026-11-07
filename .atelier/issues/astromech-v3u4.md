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
    id: "astromech-7i2k"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-nade"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-wgtr"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "The one app: astromech-core.exe runs the core, the tray and the window in one Tauri process"
updated_at: "2026-11-07T11:57:35.951474903+00:00"
---

## Goal

`astromech-core.exe` is the Tauri app. One process holds the tray, the pipeline, voice, input and, when asked, the window (decision astromech-6i8s; executable and endpoint decision in this bundle). The shell process, its watchdog and the start-the-core plumbing are gone.

**In-game behaviour (tray, kill switch, voice pause, quit):** all of these move to Tauri's event loop and must behave exactly as they do now.

## Done when

- [ ] One binary, as the executable decision says.
  - app/src-tauri's package builds the `astromech-core` binary. Its `main` is the core's command line, moved into the library (`astromech_core::cli`) with every option unchanged; `--webview-debug-port` is added from window.rs:154-157. Console mode and `--apply-update` still run before any Tauri setup (main.rs:144-184).
  - The Tauri host is built on astromech-vucf's headless assembly function, the one that builds the core from its boundaries and a `Paths`. The host adds the tray and the window around it, instead of reshaping `app::run` (app.rs:1490-1673) a second time. Only the Tauri host, not the headless build, gets a tray and a window.
  - crates/astromech-core keeps only its `sign-update` binary.
  - Without the `window` feature the binary runs headless (app.rs:1685-1698), so `cargo build --workspace` still needs no webkit2gtk. The headless binary serves scripts/e2e-gear-down.sh, scripts/mvp_criteria.py and CI's gear-down step, with their build commands updated.
  - scripts/check-deps.py and ARCHITECTURE §4 name the package.
- [ ] The tray runs on Tauri's tray API (the `tray-icon` feature), and the core's direct `tray-icon` 0.26 dependency is dropped.
  - It has the same items, texts, conditional lines, left-click and dark menus as tray.rs.
  - A pure function from the menu's state (tray.rs `Shape` and its texts, tray.rs:56-86, 102-122) to the list of items is unit-tested for each shape, so the port keeps the menu exact.
  - Dark menus (tray.rs:35-54) are applied after `Builder::build`, because tao calls `allow_dark_mode_for_app(true)` when it builds its event loop (tao-0.37.1 event_loop.rs:190).
  - `Core::notify` schedules a refresh on the main thread without waiting (`run_on_main_thread`), replacing `Ui::Refresh`.
  - No Tauri menu or window call is made while a `Core` lock is held. tray.rs:137 takes the session lock on the UI thread; that read happens before any menu call.
  - Quit, from the tray, the `quit` command or `install_update` (app.rs:540-544), calls `app.exit(0)`. The shutdown that ends `run` today, `deactivate("quit")` then `apply_on_quit` (app.rs:1669-1672), runs in `RunEvent::Exit`, so held keys are still released.
  - If the tray fails, the app logs it and runs on without it (app.rs:1675-1683).
- [ ] **Input after `build`.** The shared input source from astromech-7i2k is started, or restarted with `restart()`, after `Builder::build`, before the session can attach. tao registers keyboard and mouse Raw Input to its own hidden window when it builds the loop (tao raw_input.rs:128-157, called at event_loop.rs:200; tauri-2.12.3 app.rs:2534 sets the filter again).
  - Opening and closing the window must not register again. If the spike's check 2 showed the window does, the source registers again after each window build.
  - The device-event filter is left at its default. `DeviceEventFilter::Always` would make tao call `RIDEV_REMOVE`, which unregisters astromech's own listener too.
- [ ] The window.
  - `--background` (the Run key) creates no window and no WebView2 process. A start without it opens the window (app.rs:1664-1666).
  - `Core::open_window` builds the window, or focuses it and emits `route`. Callers are the tray, a second start through the mutex and `open_window` over the control API (app.rs:1498-1507), and an update's `--route settings` relaunch. This replaces spawning `SHELL_EXE` (app.rs:573-599).
  - Closing the window destroys it, and `ExitRequested` with no code is prevented.
  - Whatever the spike astromech-y6d6 found necessary is in, such as handling WebView2's `ProcessFailed` by destroying the dead window.
- [ ] The window's `call` runs `Registry::dispatch` in process, on `spawn_blocking`, with `origin: AppWindow`. A string `args` becomes `text`, as `control::call_window` does (crates/astromech-api/src/control.rs:192-210), and progress goes to the call's channel. The window no longer reads `core.json`.
  - Removed: window.rs:33-79 (`core_alive`, `start_core`, `watchdog`), the `core-restarted` event (named in app/src/lib/transport.ts:5's comment) and its preview entry (app/src/preview/transport.ts:43), and `tauri-plugin-single-instance`.
  - app/src/lib/ui/staleTauri.browser.test.ts:45 is updated for the in-process error text.
  - `control::call_window` goes if nothing else calls it.
  - The press listener subscribes to the previous child's input source while the window exists, and unsubscribes when the window is destroyed.
- [ ] A `local` command `ui_state` returns `{window, tray}` as in the spike, for the probes. The window's log lines, including `window: page loaded …`, go to `astromech-core.log`.
- [ ] The window's copy no longer speaks of a separate background process:
  - app/src/modules/settings/View.svelte:112;
  - the version line in app/src/modules/diagnostics/View.svelte:46-48;
  - docs/guide/getting-started.md:70 and :127.
- [ ] Packaging, so the release build and its smokes pass at this child:
  - tauri.conf.json: `mainBinaryName` is `astromech-core` and `externalBin` holds only the voice worker. scripts/stage-sidecars.sh stages only the worker.
  - .github/workflows/release.yml: the core is built by `tauri build` (`sign-update` and `mockgame` are still built separately). The smoke's file list drops `astromech-shell.exe`, and the pack-update list matches.
  - .github/workflows/ci.yml: the Windows job builds and runs clippy on the new package with `--features window`. ci.yml's gear-down step and .github/workflows/criteria.yml:61 build the headless binary from the new package.
  - scripts/onboarding_smoke.py starts `astromech-core.exe --webview-debug-port …` and finds the install by `astromech-core.exe`.
  - scripts/webview_cost.py and webview-cost.yml are retargeted to the one executable: the app is started with `--route` by a second start, and it logs to `astromech-core.log`. The "core not running" and "after a core crash" cold starts are replaced by "app not running" and "app running windowless". The fuller rework is astromech-xz7i.
- [ ] **The kill switch in one process.** The onboarding smoke's kill-switch step from astromech-7i2k passes against the one app. That step taps F8 to kill and holds F8 for 1.2 s to re-arm, with the window open, in two runs: mock game after the app, and mock game before the app. Before the app is the case where the session's listener could start before Tauri's `build`. Each run then closes the window with `WM_CLOSE` and taps F8 once more, checking that the kill switch comes on.
- [ ] `scripts/one_process_probe.py` from the spike passes against the installed app on Windows CI. It runs as a step of the Release workflow's installer job, with the endpoint `%LOCALAPPDATA%\astromech\data\core.json`. Its WebView2 match counts processes by command line as well as by descent. For the real app, check 2 (Raw Input) is covered by the onboarding smoke's kill-switch runs, so the probe skips it with `--no-sink`, and the app has no extra test hook.
- [ ] Docs:
  - ARCHITECTURE §18.1 describes one process and links astromech-6i8s; §18.6 and the naming lines at §18 (1709-1712) are updated.
  - V1-DESIGN §7.1 and §7.10 describe the one process.
  - CONTRIBUTING's layout and build lines name the new package.
- [ ] CONTRIBUTING's checks pass locally. On the PR, Windows CI, the Release build (installer smoke, onboarding smoke, probe) and App window cost pass.
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
