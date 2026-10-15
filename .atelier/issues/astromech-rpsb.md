---
created_at: "2026-10-15T20:36:13.658937507+00:00"
id: "astromech-rpsb"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Mission reminders"
updated_at: "2026-10-15T20:36:13.658937507+00:00"
---

## Goal

Missions list with expiry; warnings at 1 h and 10 min; "mission target in this system". Covers EDMC's massacre-tracker plugin.

## Done when

Replay with a `Missions` fixture warns at the right times, and an FSDJump into a target system says so.

## Context

`Missions` at login plus `Mission*` events.
