---
created_at: "2026-10-17T17:26:36.252668747+00:00"
id: "astromech-4irh"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Settle fleet transfers against the Clock, not once per journal write"
updated_at: "2026-10-17T17:29:34.219383374+00:00"
---

## Goal

Goal: the fleet (astromech_engineering::from_journal_dir) settles ship and module transfers against Utc::now() once, then the result is cached until a journal write, so a ship that has arrived can still show in transit while nothing is written. It also bypasses the Clock trait (ARCHITECTURE section 5.1).

Done when: arrivals are settled against the injected Clock whenever the Ships page reads the fleet (not only on a journal write), with a test that advances a fake clock past TransferTime and sees the ship and a fetched module arrive.

Context: found in review of PR for astromech-hnar (R7); FleetBuilder::settle(now) already takes the time.

## Done when

Done when was not specified.

## Notes

- 2026-10-17 [note] Also from the hnar review (no action there): a stored module in transit with no recorded arrival (a StoredModules refresh that reports it InTransit with no earlier fetch, or a fetch without TransferTime) never settles until the next refresh; and StorageSlot numbers are kept as the game wrote them and are not renumbered after removals, so a slot match is checked against the item name.
