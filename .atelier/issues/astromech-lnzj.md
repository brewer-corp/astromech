---
created_at: "2026-10-17T19:22:54.264627539+00:00"
id: "astromech-lnzj"
issue_type: "bug"
labels:
- "alpha"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:50:52.661902958+00:00"
status: "done"
title: "Check GuiFocus values for the left and right panels"
updated_at: "2026-10-22T21:50:52.661902958+00:00"
---

## Goal

Make sure "left panel" and "right panel" verify against the GuiFocus values the game really writes.

## Done when

GUI_LEFT and GUI_RIGHT in crates/astromech-status/src/status_file.rs match the game (checked in the game or against Frontier's journal docs), with a test.

## Context

status_file.rs has GUI_LEFT = 1 and GUI_RIGHT = 2. Frontier's journal documentation, as I recall it (not re-checked here), lists 1 = InternalPanel (right hand side) and 2 = ExternalPanel (left hand side). If that is right, "left panel" and "right panel" press the right keys but their verification waits for the other panel's value and logs "not confirmed". The mock game can't catch this because the harness uses the crate's values. Noticed while building the MVP criteria harness (astromech-08w8).

## Notes

- 2026-10-22 [note] Checked against Frontier's Player Journal Manual v38 (https://hosting.zaonce.net/community/journal/v38/Journal_Manual_v38.pdf, p. 97, 'GuiFocus values'; v30 has the same table on p. 89): 1 = InternalPanel (right hand side), 2 = ExternalPanel (left hand side). The code had them the other way round, so GUI_LEFT is now 2 and GUI_RIGHT 1, cited in status_file.rs. scripts/mvp_criteria.py had its own copy, also swapped; test_mvp_criteria.py now checks it against the crate. Not checked in the game.

## Close Reason

Done and independently reviewed on feature/astromech-0k88 (approval astromech-5ib2).
