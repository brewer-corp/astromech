---
created_at: "2026-11-07T11:57:35.950152449+00:00"
id: "astromech-nade"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T11:58:00.373496464+00:00"
status: "done"
title: "One app: its executable, single-instance guard and control endpoint"
updated_at: "2026-11-07T11:58:00.373496464+00:00"
---

## Question

The one app (astromech-y6d6) replaces two executables with one. What is that executable called, what keeps it to a single instance, and does the control API's endpoint file `core.json` stay? The choice has to be made before the first migration child, because existing installs and scripts depend on the current names:

- The Run key is `"<dir>\astromech-core.exe" --background` (crates/astromech-core/src/autostart.rs:13-15).
- The installer hooks call `astromech-core.exe --quit` (app/src-tauri/installer-hooks.nsh:5-14).
- After copying an update, the updater helper of every installed release (v1.0.0-alpha.4 today) starts `install_dir\astromech-core.exe` again (crates/astromech-core/src/update.rs:411-428). That old code runs during the upgrade, so it can't be changed.
- The named mutex `Local\astromech-core` (crates/astromech-core/src/instance.rs:4) is what stops a second core starting.
- `core.json` is read by `--quit` (main.rs:129-142), by a second start (app.rs:1498-1507), by the instance check on other platforms (instance.rs:38-45), by the Release smoke step (.github/workflows/release.yml:194-204), by scripts/onboarding_smoke.py (`core_call`, :193-207) and scripts/webview_cost.py, and by the one-command-list test. The window's "no core.json" error is tested in app/src/lib/ui/staleTauri.browser.test.ts:45 and named in app/src/lib/transport.ts:5's comment. .github/workflows/criteria.yml:61 builds the core binary. astromech-vucf's headless mode and dev bridge will use it too.

## Options

- **A. Keep `astromech-core.exe` as the one executable.** It is built by the Tauri package in app/src-tauri, so it carries the tray and the window. The mutex stays the only single-instance guard. `core.json` stays as the control API's endpoint for local clients, and the window stops reading it. The Run key, the old updater's relaunch, the installer's `--quit`, the mutex and `astromech-core.log` all keep working unchanged.
- **B. A new name (`astromech.exe`) with Tauri's single-instance plugin.** Every old release's updater would relaunch an `astromech-core.exe` that no longer exists, so a shim would be needed, and the Run key would have to be rewritten. The old core is guarded by a mutex and the new app by the plugin, so neither would see the other. An old tray core and a new app could then run together: two input listeners and two voice pipelines pressing keys in the same game.
- **C. Drop `core.json`** in favour of a fixed port or a named pipe. Every control client changes and nothing is gained: the token-in-a-file gate is what decision astromech-unh9 relies on.

## Decision

Option A, decided by the orchestrator under the strategy's Authority section (crate and module names, increment breakdown). It changes nothing the app does while playing.

- The Tauri package in app/src-tauri builds the binary `astromech-core`. The package's own name is the implementer's choice; `astromech-app` is suggested. The core's command-line handling moves into the `astromech-core` library, so that crate no longer has an `astromech-core` binary (Cargo can't put two binaries with one name in a single `target/` folder). `sign-update` stays a binary of astromech-core. Without the `window` feature the same binary runs headless, as the core does off Windows today (app.rs:1685-1698). That headless build is what Linux CI, scripts/e2e-gear-down.sh and astromech-vucf use.
- `Local\astromech-core` stays the only single-instance guard, in every version. A second start still asks the first instance, over the control API, to open the window. `tauri-plugin-single-instance` is dropped.
- `core.json` stays. Only the window's TCP hop goes: the window's `call` dispatches into the registry in process.
- **Old installs: a kept forwarder.** The installer and the update zip ship a small GUI-subsystem `astromech-shell.exe` (no console flash) that starts `astromech-core.exe` beside it with the same arguments. It keeps the Start-menu shortcuts the updater leaves, and taskbar pins to `astromech-shell.exe`, working. It is kept, not removed later:
  - `copy_payload` never deletes (update.rs:336-354), so on installs upgraded by the updater the file stays whatever a later release does;
  - removing it from the installer would break those pins.

  astromech-9lh3 files a backlog item to revisit it only with a way to move the pins.

Consequences: CONTRIBUTING's build commands, and CI's `-p astromech-core --bin astromech-core` (ci.yml, release.yml, criteria.yml:61), change to the new package.

astromech-vucf's headless assembly (one function builds the core from its boundaries and a `Paths`; world mode is `astromech-core --world <dir>` with no window) lands first. The one app builds on that function.

ARCHITECTURE §18's "the core: `astromech.exe`" placeholder is corrected.

## Report

Decided by the orchestrator under the strategy's Authority section (crate and module names):
- the one app's executable stays `astromech-core.exe`, built by the Tauri package;
- `Local\astromech-core` stays the only single-instance guard;
- `core.json` stays the control API's endpoint, and the window stops using it;
- a GUI-subsystem `astromech-shell.exe` forwarder is kept for old shortcuts and pins.

The rationale and consequences are in the Decision section.
