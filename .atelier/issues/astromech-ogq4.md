---
created_at: "2026-10-16T18:46:17.956260345+00:00"
id: "astromech-ogq4"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-ogq4
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
status: "in_progress"
title: "Companion files: stale Backpack snapshot and lost watcher notifications"
updated_at: "2026-10-17T17:16:49.008480476+00:00"
---

## Goal

Fix two gaps in the companion-file watching from astromech-9uqb. (1) Backpack.json is rewritten alongside BackpackChange with no Backpack event, so the snapshot in game state goes stale: also re-read it on BackpackChange (and check other change events). (2) A pending companion trigger is retried only when the watcher reports that exact file, so a dropped Windows notification means it is never loaded: also retry pending triggers from on_status.

## Done when

Tests show Backpack.json is re-read after a BackpackChange event, and that a pending trigger is loaded on the next status read without a file notification.

## Notes

- 2026-10-17 [note] Root cause (shared with astromech-mh3m and ztrb): the pipeline acts only when told. (1) for_event mapped only the Backpack event to Backpack.json, but the game rewrites the file with each BackpackChange and writes no Backpack event; BackpackChange now triggers the read. (2) pending triggers were retried only from a notification naming their file; on_status now retries them (quietly: a retry that fails again neither logs nor counts, so the several-per-second status reads do not spam). Not checked (needs the game): whether other events also rewrite Backpack.json or ShipLocker.json without their own event (CollectItems, DropItems, UseConsumable, Buy/Sell/Trade/TransferMicroResources); only BackpackChange is mapped.
