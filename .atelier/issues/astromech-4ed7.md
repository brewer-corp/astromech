---
created_at: "2026-10-17T06:25:51.391458322+00:00"
id: "astromech-4ed7"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-4ed7
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
closed_at: "2026-10-17T06:39:08.256858120+00:00"
status: "done"
title: "Session summary: a replay after the summary was said can let it be said again"
updated_at: "2026-10-17T06:39:08.256858120+00:00"
---

## Goal

A replay resets the session summary's "already spoken" flag: said at the menu, then a replayed journal that ends logged in, then the live Shutdown, and the summary is said a second time (finding astromech-xq9w on astromech-hk7e; confirmed by a scratch test). Keep the flag across replays once live.

## Done when

A replay test of that sequence speaks the summary once.

## Close Reason

Replay keeps the spoken flag; replay test added
