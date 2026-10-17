---
created_at: "2026-10-17T17:29:23.388191948+00:00"
id: "astromech-naa3"
issue_type: "bug"
labels:
- "alpha"
- "docs-found"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-naa3
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:46:38.568267262+00:00"
status: "done"
title: "The tray menu has no Check for updates, though the release notes and design say it does"
updated_at: "2026-10-17T19:46:38.568267262+00:00"
---

## Goal

.github/workflows/release.yml writes release notes saying later versions are installed from tray, Check for updates, and V1-DESIGN 1.2 lists check for updates in the tray menu. crates/astromech-core/src/tray.rs has only Restart to update, shown once an update is ready; Check now is in Settings, Updates.

## Done when

Either the tray gets a Check for updates item or the release notes and design text are corrected. Found while writing the user guide (docs/guide/getting-started.md describes the code).

## Close Reason

Tray item added in PR 137; unit test for the item's state; tray wiring unverified off Windows
