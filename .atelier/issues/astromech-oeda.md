---
created_at: "2026-10-15T20:21:44.875356626+00:00"
id: "astromech-oeda"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-61r5"
  - kind: "issue"
    id: "astromech-o346"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "System Powerplay on arrival"
updated_at: "2026-10-15T20:21:44.875356626+00:00"
---

## Goal

Show the controlling power, state (exploited, fortified, stronghold, acquisition), control progress, reinforcement vs undermining and contesting powers; speak it on arrival if enabled.

## Done when

An FSDJump fixture with Powerplay fields produces the expected summary, and nothing is spoken when the setting is off.

## Context

FSDJump `Powerplay*` fields. Journal only.
