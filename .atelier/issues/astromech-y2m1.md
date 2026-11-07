---
created_at: "2026-11-07T19:36:31.297883100+00:00"
id: "astromech-y2m1"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-9lh3"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "An interactive setup.exe upgrade turns Start with Windows off"
updated_at: "2026-11-07T19:48:53.597847230+00:00"
---

## Goal

Running a newer `setup.exe` interactively over an install keeps Start with Windows as the user set it, as the silent and in-app routes do.

Found reading Tauri 2.12.2's NSIS template (the installer.nsi embedded in @tauri-apps/cli 2.12.2) for astromech-9lh3; not shown on Windows. With the installer's pages shown, PageLeaveReinstall's default choice on an upgrade ("uninstall before installing") runs the old `uninstall.exe` without `/UPDATE` (only an installer started with `/UPDATE` passes it on). The old uninstaller's NSIS_HOOK_PREUNINSTALL (app/src-tauri/installer-hooks.nsh) then deletes the Run value, because `$UpdateMode <> 1`. The new app does not set it again: `autostart::apply_default` applies the default only once (`core-state.json` `autostart_default_applied`), and only updates a value that is still there. `setup.exe /S` skips the pages, so it runs no uninstaller and keeps the value (the Release workflow's upgrade job checks that route).

## Done when

- [ ] An interactive upgrade with the default choice keeps the Run value, or puts it back, shown on Windows.
- [ ] Uninstalling still removes it.

## Notes

- 2026-11-07 [observation] From finding astromech-ilwv: V1-DESIGN §7.5 said upgrades by either route leave the Run value as it was, which holds for the updater and setup.exe /S but not an interactive setup.exe upgrade until this item is fixed. The sentence now says so and names this item; when this is fixed, change §7.5 back to: every upgrade route leaves the value as it was.
