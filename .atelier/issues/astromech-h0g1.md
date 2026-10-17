---
created_at: "2026-10-17T17:29:23.534452429+00:00"
id: "astromech-h0g1"
issue_type: "task"
labels:
- "docs-found"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Plan::Press failure lines are never spoken"
updated_at: "2026-10-17T17:29:23.534452429+00:00"
---

## Goal

astromech-tools Plan::Press carries a failure line ("Landing gear did not respond.") but crates/astromech-core/src/actions.rs speaks the confirmation as soon as the key is sent and, when Status.json does not confirm within the timeout, only logs it. V1-DESIGN calls the verification silent, so either speak the failure (the command was refused by the game, which a player cannot see in VR) or delete the unused failure strings.

## Done when

The failure lines are spoken, or removed, and docs/guide/voice.md ("The game's answer") says which. Found while writing the user guide.
