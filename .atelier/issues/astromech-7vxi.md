---
created_at: "2026-11-10T05:55:44.748586055+00:00"
id: "astromech-7vxi"
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
title: "Station as a services board"
updated_at: "2026-11-10T05:55:44.748586055+00:00"
---

## Goal

A station reads like the game's services board: each service a tile, lit when present and dim when absent, with market and outfitting comparisons below as result rows.

## Done when

- [ ] (provisional) The preview station renders the board from DockedHere.services; absent services are shown, not omitted.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockup: the station block of docs/design/glow-up-now.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

The full service list to show as absent, and how this shares with the Now page's docked state.
