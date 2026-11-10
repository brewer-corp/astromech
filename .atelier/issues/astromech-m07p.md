---
created_at: "2026-11-10T05:55:43.303344530+00:00"
id: "astromech-m07p"
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
title: "Colonisation as progress per commodity"
updated_at: "2026-11-10T05:55:43.303344530+00:00"
---

## Goal

A construction site reads as progress: a bar per commodity (delivered against needed, aboard marked), loads left for the current ship, and where to buy as the finder below.

## Done when

- [ ] (provisional) The preview site renders bars that match its numbers.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc).  Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

How several sites are switched between.
