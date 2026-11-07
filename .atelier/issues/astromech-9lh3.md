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
status: "in_progress"
title: "Installer, updater and Start with Windows on the one executable; upgrade from the last two-process release on Windows CI"
updated_at: "2026-11-07T19:40:17.586788423+00:00"
---

## Goal

The installer, the updater and Start with Windows work with the one executable. An existing two-process install (the newest release, v1.0.0-alpha.4 today) upgrades cleanly by either route: the NSIS installer run over it while it runs, and the in-app updater. The updater route is the old release's own helper, which copies the update zip over the program folder and relaunches `astromech-core.exe` (crates/astromech-core/src/update.rs:411-428, 505-551). That helper is old code, so this release has to fit it.

**In-game behaviour:** an update while playing is unchanged (quit releasing keys, copy, relaunch tray-only). Check the NSIS order below, because an installer that force-kills the app first would skip the clean quit.

## Done when

- [ ] The forwarder, kept as decision astromech-nade says, is a tiny bin crate without Tauri, built as `astromech-shell.exe`. It starts `astromech-core.exe` from its own folder with its own arguments and exits.
  - It is GUI-subsystem (`windows_subsystem = "windows"`), so it flashes no console window.
  - It ships through `externalBin` (scripts/stage-sidecars.sh) and in the update zip (scripts/pack-update.py's list in release.yml).
  - A unit test covers how the command line is built.
  - A backlog item is filed (`--status backlog`, discovered from this one). It records that the forwarder stays: `copy_payload` never deletes (update.rs:336-354), so on updater-route installs the old file is there whatever a later release ships, and removing the forwarder would break taskbar pins to `astromech-shell.exe`. Revisit only with a way to move those pins.
- [ ] app/src-tauri/installer-hooks.nsh's ASTROMECH_STOP_CORE also ends `astromech-shell.exe`, an old release's window. `--quit` still comes first.
  - The upgrade job below shows whether Tauri's NSIS template force-kills the main binary before NSIS_HOOK_PREINSTALL runs.
  - If it does, the hook asks the app to `--quit` before that kill, or the Close Reason records why it can't.
- [x] update.rs:
  - `install_update` picks `RelaunchWithWindow` from the in-process window state, not from a process named `astromech-shell.exe` (app.rs:529);
  - `stop_locked_exes` (update.rs:485-497) and the tests that name `astromech-shell.exe` (update.rs:660-695) match the new layout.
- [ ] The Run value stays `"<dir>\astromech-core.exe" --background` (autostart.rs:13-15), and `apply_default` keeps the path current (autostart.rs:38-57; the program-path update is at :41-48). The installer smoke reads it back.
- [x] The retired `astromech-shell.log` and its rolled files are pruned after 7 days, like the others (crates/astromech-log `prune`, which keeps a live file today, lib.rs:109-115). A unit test covers it.
- [ ] The Release workflow's install smoke asserts, after `--background`:
  - exactly one `astromech-core` process, and no `astromech-shell` process;
  - no `msedgewebview2.exe` belongs to the app, matched by descent and by command line (`--webview-exe-name=astromech-core.exe` or the app's `EBWebView` folder), as the probe does;
  - a second start leaves one process;
  - the Run value is set;
  - `--quit` ends it and logs `quitting`;
  - uninstalling removes the Run value.
- [ ] A new `upgrade` job in .github/workflows/release.yml uses `needs: installer` and the installer artifact, on `windows-latest`. It installs the newest `v*` release with `gh release download` and `/S`, and starts it as a player has it: the core with `--background`, then the old window. Then it runs two routes from that state:
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

## Report

Implementer report (agent-factory implement; migrate's expand step for two-process installs, whose old helper and uninstaller are code this release can't change). Committed on astromech-y6d6; Windows CI not yet run on it.

Outcome against Done when. Ticked, proven locally: update.rs (`install_update` already picks `RelaunchWithWindow` from `ui.window_open()`, app.rs:706, read; `stop_locked_exes` uses `LOCKED_EXES` and the tests no longer use `astromech-shell.exe` as a sample name; `an_update_over_a_two_process_install_replaces_its_window_with_the_forwarder` passes); the retired-log prune (`the_retired_window_log_is_pruned_after_7_days` and `the_start_prunes_the_retired_logs` pass, and both fail with the live file kept); V1-DESIGN §7.2, §7.3, §7.5, §7.7 (and §7.8's build and upgrade lines). Built, and waiting on Windows CI: the forwarder (crates/astromech-shell, GUI subsystem in every build, externalBin through scripts/stage-sidecars.sh, required in the update zip; `starts_the_core_beside_it_with_its_own_arguments` and `a_start_with_no_arguments_passes_none` pass; backlog item astromech-9e63 filed); the hook (`--quit`, then `astromech-shell.exe`, then the forced fallback); the Run value read back exactly; the install smoke's new assertions; the `upgrade` job (scripts/upgrade_smoke.py). The needs-human line stays open.

NSIS kill order (the v3u4 review's question), from the template embedded in @tauri-apps/cli 2.12.2 (the version app/pnpm-lock.yaml pins, and the one v1.0.0-alpha.4 was built with): `Section Install` runs `NSIS_HOOK_PREINSTALL` first, then `CheckIfAppIsRunning` on `$INSTDIR\astromech-core.exe`, which is the Restart Manager's `RmShutdown(RmForceShutdown)`; `Section Uninstall` has the same order around `NSIS_HOOK_PREUNINSTALL`. Nothing before them kills the app: `.onInit` and the WebView2 section don't, and under `/S` the reinstall page (which would run the old uninstaller) is skipped. So the hook's `--quit` is always first. `--quit` returns once `core.json` is gone, which the app removes only after `shutdown` (`deactivate`, whose `policy.kill()` releases held keys first); only a core that takes over 5 s to shut down can still meet the hook's own `taskkill /F`, after its keys were released. The upgrade job's NSIS route shows it on Windows: the old core must log `quitting` and remove `core.json` during `setup.exe /S`. Nothing needed changing for held keys.

Proof (Linux, after the last change): `cargo test --workspace --locked` 1323 passed, 0 failed; `cargo clippy --workspace --all-targets --locked -D warnings` clean; Windows-target clippy (fake cl, lib, llvm-rc) clean for the workspace and for `astromech-app --features window` (a planted type error in host.rs's `state` fails it); `cargo fmt --all --check`; `scripts/check-deps.py` (33 crates); `pnpm check` 0 errors; bindings test passes after regenerating UiState.ts; actionlint 1.7.7 and PyYAML on release.yml and ci.yml clean; py_compile and pyflakes on scripts/upgrade_smoke.py clean; `stage-sidecars.sh` with `PLACEHOLDER=1` stages both files; the job's `gh release list … --jq` picks v1.0.0-alpha.4 against the real repository, whose release has `astromech_1.0.0-alpha.4_x64-setup.exe`. Not run: makensis (not on this host), and nothing of the installer, the forwarder or the upgrade script on Windows.

### Deviations

- `ui_state` gained `page` (decision note), for the updater route's "open at Settings" check.
- The NSIS route starts the app from the Start-menu shortcut after `setup.exe /S`, which starts nothing itself (decision note).
- `publish` now needs `upgrade` (decision note).
- ARCHITECTURE (§4's crate list, §18.1) and CONTRIBUTING (the Release build lines) name the forwarder and the upgrade job.

### Surprises

- Tauri's template deletes the previously registered main binary (`MainBinaryName`, `astromech-shell.exe` on alpha.4) after copying the files, which would have deleted the forwarder on every NSIS upgrade from alpha.4. The PREINSTALL hook re-registers the name first (decision note).
- An interactive `setup.exe` upgrade runs the old uninstaller without `/UPDATE`, which deletes the Run value for good (filed as astromech-y2m1; not on either route tested here).

### Open items

- CI: the Release build's installer smoke (forwarder installed, one process, no `astromech-shell`, no WebView2, the exact Run value, `quitting`), and the `upgrade` job's two routes. Lines 1, 2, 4, 6 and 7 wait for them.
- Independent review is outstanding.
- The hook change has not been through makensis here; the first Release build is its compile.
- astromech-9e63 (backlog, the forwarder's removal) and astromech-y2m1 (interactive upgrade and Start with Windows) are new.
