---
created_at: "2026-10-17T08:01:20.771980573+00:00"
id: "astromech-lelp"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-lelp
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
closed_at: "2026-10-17T08:46:06.504998238+00:00"
status: "done"
title: "System view from history: honk counts, and only the newest scan per body"
updated_at: "2026-10-17T08:46:06.504998238+00:00"
---

## Goal

Follow-ups on astromech-0n4h: the body count and 'all bodies found' flag from an earlier visit are always empty because the history lookup never selects FSSDiscoveryScan and FSSAllBodiesFound (finding astromech-1pec); in a heavily visited system the star's scan is stored on every arrival and re-read on every poll; keep only the newest scan per body (finding astromech-qy9j).

## Done when

A return visit shows the earlier body count and all-found flag; a system with 50 stored arrival scans reads one per body (test).

## Close Reason

Earlier honk count and all-found flag now selected; newest scan/mapping per body only; v7 index
