---
created_at: "2026-10-16T21:01:46.443277199+00:00"
id: "astromech-xvov"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7ni2"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Powerplay: incremental replay and state edge cases"
updated_at: "2026-10-22T21:20:25.277060911+00:00"
---

## Goal

The Powerplay page re-reads every journal about every 2 s during play (journal_version changes on each append); a login without a Powerplay event (not pledged) leaves a stale power if the leave journal was deleted; a system with a Powerplay state but no controlling power is announced as Unoccupied; the undermine re-check doesn't re-check the state.

## Done when

Incremental replay (from the last offset, or files since last week plus the newest snapshot); login without Powerplay clears the pledge; state shown when there is no controller; undermine re-check covers the state; tests.

## Context

Found in review of astromech-7ni2 (R1, R2, R4, R5). R3 (rank/merits reset on defection) and community-sourced field semantics belong to the game check astromech-w1q2.
