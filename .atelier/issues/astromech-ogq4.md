---
created_at: "2026-10-16T18:46:17.956260345+00:00"
id: "astromech-ogq4"
issue_type: "task"
labels:
- "alpha"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-tpdb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Companion files: stale Backpack snapshot and lost watcher notifications"
updated_at: "2026-10-17T17:01:40.698901406+00:00"
---

## Goal

Fix two gaps in the companion-file watching from astromech-9uqb. (1) Backpack.json is rewritten alongside BackpackChange with no Backpack event, so the snapshot in game state goes stale: also re-read it on BackpackChange (and check other change events). (2) A pending companion trigger is retried only when the watcher reports that exact file, so a dropped Windows notification means it is never loaded: also retry pending triggers from on_status.

## Done when

Tests show Backpack.json is re-read after a BackpackChange event, and that a pending trigger is loaded on the next status read without a file notification.
