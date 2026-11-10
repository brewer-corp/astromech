---
created_at: "2026-11-10T05:55:45.472196717+00:00"
id: "astromech-el3i"
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
title: "Trade route as legs"
updated_at: "2026-11-10T05:55:45.472196717+00:00"
---

## Goal

A found trade route reads as its legs: station, what to buy, where to sell, profit per leg and per hour, on the Workbench layout's result side. Today the result side is an empty page until a search returns.

## Done when

- [ ] (provisional) Single hop and loop both render as legs from the existing reply.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc).  Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

None known.
