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
closed_at: "2026-10-17T17:16:52.071152408+00:00"
status: "done"
title: "Companion files: stale Backpack snapshot and lost watcher notifications"
updated_at: "2026-10-17T18:08:08.663811692+00:00"
---

## Goal

Fix two gaps in the companion-file watching from astromech-9uqb. (1) Backpack.json is rewritten alongside BackpackChange with no Backpack event, so the snapshot in game state goes stale: also re-read it on BackpackChange (and check other change events). (2) A pending companion trigger is retried only when the watcher reports that exact file, so a dropped Windows notification means it is never loaded: also retry pending triggers from on_status.

## Done when

Tests show Backpack.json is re-read after a BackpackChange event, and that a pending trigger is loaded on the next status read without a file notification.

## Notes

- 2026-10-17 [note] Root cause (shared with astromech-mh3m and ztrb): the pipeline acts only when told. (1) for_event mapped only the Backpack event to Backpack.json, but the game rewrites the file with each BackpackChange and writes no Backpack event; BackpackChange now triggers the read. (2) pending triggers were retried only from a notification naming their file; on_status now retries them (quietly: a retry that fails again neither logs nor counts, so the several-per-second status reads do not spam). Not checked (needs the game): whether other events also rewrite Backpack.json or ShipLocker.json without their own event (CollectItems, DropItems, UseConsumable, Buy/Sell/Trade/TransferMicroResources); only BackpackChange is mapped.
- 2026-10-17 [note] O4: which other events rewrite Backpack.json or ShipLocker.json without their own event is covered by astromech-t8hg (the game check). Review follow-ups: retries now run after the journal read (never ahead of the batch's lines), also from the idle scan, and give up on an event older than five minutes (logged once).

## Close Reason

BackpackChange now triggers a Backpack.json read; Tailer::retry_pending runs on every status read. Tests backpack_json_is_read_again_after_a_backpack_change and a_pending_companion_event_is_loaded_on_the_next_status_read (mutation-checked). Other events that may rewrite the files are unchecked (needs the game). Review outstanding.
