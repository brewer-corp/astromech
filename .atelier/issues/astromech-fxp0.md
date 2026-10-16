---
created_at: "2026-10-15T20:36:13.653039260+00:00"
id: "astromech-fxp0"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-tqdh"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:11:56.522186501+00:00"
status: "done"
title: "Logbook"
updated_at: "2026-10-16T20:11:56.522186501+00:00"
---

## Goal

Visited systems and stations, first and last visit, count, timeline, and full-text search.

## Done when

For the fixture history, a search finds a visited station with the right counts.

## Close Reason

Logbook: places with counts and first/last visit, totals, timeline, substring search over system/station names; History::logbook + read-only shell command + Logbook tab with SystemChip. Fixture search 'hutton' finds Hutton Orbital with 2 visits (Rust + vitest). 27e5c4b. Shell compile is on Windows CI.
