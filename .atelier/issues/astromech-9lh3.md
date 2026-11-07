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
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Installer, updater and Start with Windows on the one executable; upgrade from the last two-process release on Windows CI"
updated_at: "2026-11-07T11:57:35.952798610+00:00"
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
- [ ] update.rs:
  - `install_update` picks `RelaunchWithWindow` from the in-process window state, not from a process named `astromech-shell.exe` (app.rs:529);
  - `stop_locked_exes` (update.rs:485-497) and the tests that name `astromech-shell.exe` (update.rs:660-695) match the new layout.
- [ ] The Run value stays `"<dir>\astromech-core.exe" --background` (autostart.rs:13-15), and `apply_default` keeps the path current (autostart.rs:38-57; the program-path update is at :41-48). The installer smoke reads it back.
- [ ] The retired `astromech-shell.log` and its rolled files are pruned after 7 days, like the others (crates/astromech-log `prune`, which keeps a live file today, lib.rs:109-115). A unit test covers it.
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
- [ ] V1-DESIGN §7.2 (what the installer carries), §7.3 (the payload list), §7.5 and §7.7 (single instance, logs) describe the new layout.
- [ ] After the next release (needs-human: Sumeet's PC). From his two-process install, Settings → Install and restart brings back one `astromech-core.exe`. His Start-menu and pinned shortcuts open the window, and at the next login Start with Windows shows the tray only. Stand-in: the `upgrade` job.

## Scope

app/src-tauri (hooks, conf), the forwarder crate, crates/astromech-core (update.rs, app.rs `install_update`), crates/astromech-log, scripts/stage-sidecars.sh, scripts/pack-update.py's inputs, .github/workflows/release.yml, and the V1-DESIGN sections named above.

## Context

- Decision astromech-nade.
- The NSIS upgrade runs the old uninstaller with `/UPDATE` (installer-hooks.nsh:22-25).
- `copy_payload` overlays and never deletes (update.rs:336-354). So an updater upgrade leaves the old `astromech-shell.exe` behind until the forwarder replaces it.
- Both the old shell and the one app use Tauri's identifier `io.github.spadaval.astromech`, so the WebView2 profile, and with it the window's saved state, carries over.
