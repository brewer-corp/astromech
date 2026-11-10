---
created_at: "2026-11-10T05:55:40.418245755+00:00"
id: "astromech-evj3"
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
title: "Exploration > Biology as a card per body"
updated_at: "2026-11-10T05:55:40.418245755+00:00"
---

## Goal

Each body with biology is a card: its genera with values, sample progress as pips (1 of 3), and while sampling a gauge of distance against the colony range. It replaces two tables.

## Done when

- [ ] (provisional) The sampling state and both body lists render as cards from the existing replies.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc).  Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

Whether 'reported by other players' merges into the same cards or stays a second group.
