---
created_at: "2026-11-07T19:36:19.929914736+00:00"
id: "astromech-9e63"
issue_type: "task"
labels: []
priority: "P3"
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
status: "backlog"
title: "Revisit the astromech-shell.exe forwarder only with a way to move taskbar pins"
updated_at: "2026-11-07T19:36:19.929914736+00:00"
---

## Goal

Decide whether the `astromech-shell.exe` forwarder (crates/astromech-shell, decision astromech-nade) can ever go. Until then it stays in the installer and the update zip.

It stays because:
- the updater's `copy_payload` never deletes (crates/astromech-core/src/update.rs, `copy_payload`), so on installs upgraded through the in-app updater the old file is there whatever a later release ships: dropping it from the zip would leave the two-process release's window behind, not remove it;
- removing it from the installer would break taskbar pins to `astromech-shell.exe`, which Tauri's template does not retarget (it moves only the Start-menu and desktop shortcuts).

Revisit only with a way to move those pins (or evidence that no install still has one).

## Done when

- [ ] A way to move taskbar pins from `astromech-shell.exe` to `astromech-core.exe` is found and shown on Windows, or the forwarder is confirmed as kept for good and this item closed with that reason.
