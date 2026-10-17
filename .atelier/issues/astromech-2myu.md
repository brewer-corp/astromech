---
created_at: "2026-10-15T20:36:13.658518406+00:00"
id: "astromech-2myu"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-2myu
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-tqdh"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T02:43:00.365098051+00:00"
status: "done"
title: "Visit history in the arrival briefing"
updated_at: "2026-10-17T02:43:00.365098051+00:00"
---

## Goal

Add "times visited" and "last here …" to the arrival briefing from the history ledgers.

## Done when

Replay with a fixture history: a system visited twice before gets the line; a new system doesn't.

## Close Reason

Arrival briefing says 'Visited N times. Last here X ago.' from the visits ledger; first visits and catch-up are silent.
