---
created_at: "2026-10-17T05:47:48.456521773+00:00"
id: "astromech-hk7e"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-hk7e
priority: "P3"
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
closed_at: "2026-10-17T06:20:36.732977236+00:00"
status: "done"
title: "Session summary: a replay during the wait drops a pending summary"
updated_at: "2026-10-17T06:20:36.732977236+00:00"
---

## Goal

`Rule::replayed` resets the session summary's whole state on every replayed line, including replays after going live, so a replay during the 30-second wait for the history drops a pending summary (finding astromech-s5ji on astromech-627q). Keep a pending summary across replays.

## Done when

A replay test with a pending summary and a replayed line before the history catches up still speaks the summary once.

## Close Reason

Replays keep a pending summary; replay test passes
