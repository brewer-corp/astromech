---
created_at: "2026-11-10T05:55:44.022030945+00:00"
id: "astromech-fzhp"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-irtr"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Session and Logbook as time"
updated_at: "2026-11-10T05:55:44.022030945+00:00"
---

## Goal

A session reads as earnings over its length by activity, and the logbook as travel grouped by day, in place of a table of payments and a table of timestamps.

## Done when

- [ ] (provisional) Both render from the existing replies; the logbook search still works.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc).  Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

Whether Session's reply carries per-payment times (it has totals by kind today); if not, the core adds them.
