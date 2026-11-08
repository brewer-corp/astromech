---
created_at: "2026-11-07T11:57:35.952798610+00:00"
id: "astromech-9lh3"
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
    id: "astromech-nade"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-v3u4"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T15:51:21.522469461+00:00"
status: "done"
title: "Installer, updater and Start with Windows on the one executable; upgrade from the last two-process release on Windows CI"
updated_at: "2026-11-08T15:51:21.522469461+00:00"
---

## Goal

The installer, the updater and Start with Windows work with the one executable. An existing two-process install (the newest release, v1.0.0-alpha.4 today) upgrades cleanly by either route: the NSIS installer run over it while it runs, and the in-app updater. The updater route is the old release's own helper, which copies the update zip over the program folder and relaunches `astromech-core.exe` (crates/astromech-core/src/update.rs:411-428, 505-551). That helper is old code, so this release has to fit it.

**In-game behaviour:** an update while playing is unchanged (quit releasing keys, copy, relaunch tray-only). Check the NSIS order below, because an installer that force-kills the app first would skip the clean quit.

## Done when

- [x] The forwarder, kept as decision astromech-nade says, is a tiny bin crate without Tauri, built as `astromech-shell.exe`. It starts `astromech-core.exe` from its own folder with its own arguments and exits.
  - It is GUI-subsystem (`windows_subsystem = "windows"`), so it flashes no console window.
  - It ships through `externalBin` (scripts/stage-sidecars.sh) and in the update zip (scripts/pack-update.py's list in release.yml).
  - A unit test covers how the command line is built.
  - A backlog item is filed (`--status backlog`, discovered from this one). It records that the forwarder stays: `copy_payload` never deletes (update.rs:336-354), so on updater-route installs the old file is there whatever a later release ships, and removing the forwarder would break taskbar pins to `astromech-shell.exe`. Revisit only with a way to move those pins.
- [x] app/src-tauri/installer-hooks.nsh's ASTROMECH_STOP_CORE also ends `astromech-shell.exe`, an old release's window. `--quit` still comes first.
  - The upgrade job below shows whether Tauri's NSIS template force-kills the main binary before NSIS_HOOK_PREINSTALL runs.
  - If it does, the hook asks the app to `--quit` before that kill, or the Close Reason records why it can't.
- [x] update.rs:
  - `install_update` picks `RelaunchWithWindow` from the in-process window state, not from a process named `astromech-shell.exe` (app.rs:529);
  - `stop_locked_exes` (update.rs:485-497) and the tests that name `astromech-shell.exe` (update.rs:660-695) match the new layout.
- [x] The Run value stays `"<dir>\astromech-core.exe" --background` (autostart.rs:13-15), and `apply_default` keeps the path current (autostart.rs:38-57; the program-path update is at :41-48). The installer smoke reads it back.
- [x] The retired `astromech-shell.log` and its rolled files are pruned after 7 days, like the others (crates/astromech-log `prune`, which keeps a live file today, lib.rs:109-115). A unit test covers it.
- [x] The Release workflow's install smoke asserts, after `--background`:
  - exactly one `astromech-core` process, and no `astromech-shell` process;
  - no `msedgewebview2.exe` belongs to the app, matched by descent and by command line (`--webview-exe-name=astromech-core.exe` or the app's `EBWebView` folder), as the probe does;
  - a second start leaves one process;
  - the Run value is set;
  - `--quit` ends it and logs `quitting`;
  - uninstalling removes the Run value.
- [x] A new `upgrade` job in .github/workflows/release.yml uses `needs: installer` and the installer artifact, on `windows-latest`. It installs the newest `v*` release with `gh release download` and `/S`, and starts it as a player has it: the core with `--background`, then the old window. Then it runs two routes from that state:
  1. **Updater.** Extract this build's update zip to a folder. Copy the old `astromech-core.exe` to `%TEMP%\astromech-apply-<pid>.exe`, the name `spawn_apply` uses (update.rs:449-455). Under its own name, the helper's `taskkill /F /T /IM astromech-core.exe` (`stop_locked_exes`, update.rs:485-497) would kill the helper itself. Run it with `--apply-update --from <folder> --install-dir <inst> --wait-pid <old core pid> --relaunch --route settings`, which is `RelaunchWithWindow`, the case with the old window open (app.rs:529-530, update.rs:465-472). Then `--quit` the old core, as the core's install does. The signed-feed download before that is unchanged old code and isn't part of the test.
  2. **NSIS.** Reinstall the old release fresh, start it, and run this build's `setup.exe /S` while it runs.

  After each route, within 30 s:
  - exactly one `astromech-core.exe` is running, its `status.version` is this build's, and no `astromech-shell.exe` process is left;
  - after the updater route, the window is open at Settings in that process (`ui_state`);
  - the Run value is unchanged, and `core.json` answers;
  - starting `<inst>\astromech-shell.exe` (an old shortcut) opens the window in the running app, with one process and a page-load line;
  - after the NSIS route, the Start-menu shortcut targets `astromech-core.exe`.

  The job ends by uninstalling.
- [x] V1-DESIGN §7.2 (what the installer carries), §7.3 (the payload list), §7.5 and §7.7 (single instance, logs) describe the new layout.
- [ ] After the next release (needs-human: Sumeet's PC). From his two-process install, Settings → Install and restart brings back one `astromech-core.exe`. His Start-menu and pinned shortcuts open the window, and at the next login Start with Windows shows the tray only. Stand-in: the `upgrade` job.

## Scope

app/src-tauri (hooks, conf), the forwarder crate, crates/astromech-core (update.rs, app.rs `install_update`), crates/astromech-log, scripts/stage-sidecars.sh, scripts/pack-update.py's inputs, .github/workflows/release.yml, and the V1-DESIGN sections named above.

## Context

- Decision astromech-nade.
- The NSIS upgrade runs the old uninstaller with `/UPDATE` (installer-hooks.nsh:22-25).
- `copy_payload` overlays and never deletes (update.rs:336-354). So an updater upgrade leaves the old `astromech-shell.exe` behind until the forwarder replaces it.
- Both the old shell and the one app use Tauri's identifier `io.github.spadaval.astromech`, so the WebView2 profile, and with it the window's saved state, carries over.

## Notes

- 2026-11-07 [decision] NSIS_HOOK_PREINSTALL re-registers MainBinaryName as astromech-core.exe when it reads astromech-shell.exe, so Tauri's template keeps the forwarder
  Reason: Tauri 2.12.2's Section Install deletes $INSTDIR\$OldMainBinaryName after copying the files when the registered name differs; v1.0.0-alpha.4 (same CLI) registered astromech-shell.exe, so setup.exe /S would install the forwarder and then delete it, breaking pins. Without /UPDATE the Start-menu and desktop shortcuts are written again on astromech-core.exe, so the template's shortcut migration isn't needed
  Rejected: Ship a second copy of the forwarder as a resource and copy it back in POSTINSTALL: two copies and a file the uninstaller must also know
- 2026-11-07 [decision] ASTROMECH_STOP_CORE: --quit, then taskkill astromech-shell.exe, then the forced fallback for astromech-core.exe and the worker
  Reason: An old window's watchdog restarts the core when core.json is there but the core doesn't answer, which is what a forced kill leaves; after a clean quit core.json is gone and the watchdog stays idle, so the old window goes between the two
- 2026-11-07 [decision] ui_state gains page (the window URL's #/ route, astromech_core::app::page_of) so the upgrade job can check the updater's --route settings relaunch
  Reason: The Done when line checks the window open at Settings via ui_state, which had only window and tray; the log has no line for the first page
- 2026-11-07 [decision] The upgrade job's NSIS route starts the app from the Start-menu shortcut after setup.exe /S
  Reason: A silent NSIS install starts nothing afterwards (the template's RunMainBinary is the finish page's); a player starts it from the shortcut, and if an old core had survived the install the shortcut's start would hand over to it and the version check would fail
- 2026-11-07 [decision] The Release workflow's publish job needs the upgrade job
  Reason: CONTRIBUTING says a release fails before publishing if any step fails; a build that two-process installs can't upgrade to should not reach the updater feed
  Rejected: Leave publish on installer only: a broken upgrade would ship
- 2026-11-07 [decision] The NSIS route's clean-quit check is the old core's quitting line plus core.json removed; its exit code is reported, not required
  Reason: core.json goes after shutdown releases held keys, so its absence proves the clean quit finished; the hook's fallback taskkill can still hit a core between removing core.json and exiting (exit code 1, harmless), so requiring 0 would flake. On the updater route the helper waits for the old core to exit, so code 0 is required there
- 2026-11-07 [handoff] Built at f6a048be; see ## Report. Ticked: update.rs, log prune, V1-DESIGN. Waiting on Windows CI (Release build installer smoke and the new upgrade job): lines 1, 2, 4, 6, 7. Independent review outstanding.
- 2026-11-07 [resolution] Findings kfub and s3rq fixed: upgrade_smoke.py's call() takes a bare reply (alpha.4's control server sends {ok, pid, version} with no value wrapper), which is every reply from the old core the script reads (status in old_running); the upgrade job pins v1.0.0-alpha.4, the last two-process release, and astromech-7pl5 covers upgrades between one-process releases. Also: V1-DESIGN §7.5 no longer claims every route keeps the Run value (finding ilwv, note on y2m1); the LOCKED_EXES comment names the /T kill of the helper (finding 81t0, backlog astromech-mqqa)
- 2026-11-07 [result] Upgrade job at 0609f29e (run 42410580322): updater route passed every check up to 'the window is open at Settings', which read {page: None, window: open}. Cause: the script read ui_state 0.3 s after the relaunch, before the window's first page load (the route's core log ends at 'opened the app window at settings' with no 'window: page loaded' line); ui_state's page is WebView2's Source, about:blank until the first navigation commits, while window counts as open once the window is built. The route did reach the window (index.html#/settings is what open_window builds, and App.svelte keeps a #/settings start). Fix: the script waits for the page-load line, then for page; page_of now takes the whole URL and is tested on http://tauri.localhost/…#/settings, tauri://localhost/…#/settings and about:blank; UiState documents the wait. The script now runs the NSIS route after an updater-route failure and fails the run if either failed (checked with a stubbed run: updater raises, nsis still runs, exit 1).

## Report

Implementer report (agent-factory implement; migrate's expand step for two-process installs, whose old helper and uninstaller are code this release can't change). Committed on astromech-y6d6 (f6a048be, fixes 74e8546b and 98daaece); PR #218 passed every check at 98daaece (evidence astromech-ym5y).

Outcome against Done when: every line holds except the needs-human one (Sumeet's own install after the next release; stand-in: the upgrade job). Proven locally: update.rs (`install_update` already picks `RelaunchWithWindow` from `ui.window_open()`, app.rs:706, read; `stop_locked_exes` uses `LOCKED_EXES`; `an_update_over_a_two_process_install_replaces_its_window_with_the_forwarder` passes); the retired-log prune (`the_retired_window_log_is_pruned_after_7_days` and `the_start_prunes_the_retired_logs` pass, and both fail with the live file kept); V1-DESIGN §7.2, §7.3, §7.5, §7.7 (and §7.8). Proven on Windows CI, Release build run 42412634644 (evidence astromech-ym5y):
- the forwarder (crates/astromech-shell; `starts_the_core_beside_it_with_its_own_arguments` and `a_start_with_no_arguments_passes_none` pass; backlog astromech-9e63) is built, installed by the NSIS installer (the installer smoke's file list) and carried in the zip, and on both routes `astromech-shell.exe` exits 0 and opens the window in the running app;
- the hook: on the NSIS route the old core exited 0 after logging `quitting` and removing `core.json`, and the installer left no `astromech-core` or `astromech-shell` running;
- the Run value: read back exactly by the installer smoke and unchanged by both routes (`apply_default`'s path update is unchanged code, not exercised by CI);
- the installer smoke's assertions: one `astromech-core`, no `astromech-shell`, no WebView2 of the app's among 140 processes, one process after a second start, `quitting` logged, the Run value removed by uninstall;
- the `upgrade` job: both routes from v1.0.0-alpha.4 passed every check, including `ui_state` `{page: settings, window: open}` after the updater route and the Start-menu shortcut on `astromech-core.exe` after the NSIS route; it ended uninstalled.

The upgrade line asks for "the newest `v*` release"; the job pins v1.0.0-alpha.4 instead, by finding astromech-s3rq (the newest release stops being two-process once this one ships), with astromech-7pl5 for upgrades between one-process releases.

NSIS kill order (the v3u4 review's question), from the template embedded in @tauri-apps/cli 2.12.2 (the version app/pnpm-lock.yaml pins, and the one v1.0.0-alpha.4 was built with): `Section Install` runs `NSIS_HOOK_PREINSTALL` first, then `CheckIfAppIsRunning` on `$INSTDIR\astromech-core.exe`, which is the Restart Manager's `RmShutdown(RmForceShutdown)`; `Section Uninstall` has the same order around `NSIS_HOOK_PREUNINSTALL`. Nothing before them kills the app: `.onInit` and the WebView2 section don't, and under `/S` the reinstall page (which would run the old uninstaller) is skipped. So the hook's `--quit` is always first. `--quit` returns once `core.json` is gone, which the app removes only after `shutdown` (`deactivate`, whose `policy.kill()` releases held keys first); only a core that takes over 5 s to shut down can still meet the hook's own `taskkill /F`, after its keys were released. The upgrade job's NSIS route showed it on Windows (run 42412634644): during `setup.exe /S` the old core logged `quitting`, removed `core.json` and exited with code 0, which a forced stop (taskkill /F, RmShutdown) would have made 1. Nothing needed changing for held keys.

Proof (Linux, after the last change): `cargo test --workspace --locked` 1323 passed, 0 failed; `cargo clippy --workspace --all-targets --locked -D warnings` clean; Windows-target clippy (fake cl, lib, llvm-rc) clean for the workspace and for `astromech-app --features window` (a planted type error in host.rs's `state` fails it); `cargo fmt --all --check`; `scripts/check-deps.py` (33 crates); `pnpm check` 0 errors; bindings test passes after regenerating UiState.ts; actionlint 1.7.7 and PyYAML on release.yml and ci.yml clean; py_compile and pyflakes on scripts/upgrade_smoke.py clean; `stage-sidecars.sh` with `PLACEHOLDER=1` stages both files; the job's old release is pinned to v1.0.0-alpha.4 (finding astromech-s3rq), whose release has `astromech_1.0.0-alpha.4_x64-setup.exe`. Not run here: makensis (not on this host); the installer, the forwarder and the upgrade script ran on Windows CI instead (above).

### Deviations

- `ui_state` gained `page` (decision note), for the updater route's "open at Settings" check. It reads WebView2's URL, which is `about:blank` until the first page loads, so the script waits for the page-load line first (the first CI run read it 0.3 s after the relaunch and got None; fixed in 98daaece).
- The upgrade job pins v1.0.0-alpha.4 (finding astromech-s3rq), and the script runs both routes even when one fails.
- The NSIS route starts the app from the Start-menu shortcut after `setup.exe /S`, which starts nothing itself (decision note).
- `publish` now needs `upgrade` (decision note).
- ARCHITECTURE (§4's crate list, §18.1) and CONTRIBUTING (the Release build lines) name the forwarder and the upgrade job.

### Surprises

- Tauri's template deletes the previously registered main binary (`MainBinaryName`, `astromech-shell.exe` on alpha.4) after copying the files, which would have deleted the forwarder on every NSIS upgrade from alpha.4. The PREINSTALL hook re-registers the name first (decision note).
- An interactive `setup.exe` upgrade runs the old uninstaller without `/UPDATE`, which deletes the Run value for good (filed as astromech-y2m1; not on either route tested here).

### Open items

- The needs-human line: Sumeet's own two-process install, after the next release.
- `apply_default`'s program-path update is not exercised on Windows CI (unchanged code).
- New: astromech-9e63 (backlog, the forwarder's removal), astromech-y2m1 (interactive upgrade and Start with Windows), astromech-7pl5 (upgrade test between one-process releases), astromech-mqqa (backlog, the helper and `taskkill /T`).
