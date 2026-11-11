---
schema: atelier.item
schema_version: 3
id: astromech-7vxi
title: Station as a services board
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-irtr
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:44.748586055Z
---

## Goal

A station reads like the game's services board: each service a tile, lit when present and dim when absent, with market and outfitting comparisons below as result rows.

## Done when

- [ ] (provisional) The preview station renders the board from DockedHere.services; absent services are shown, not omitted.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockup: the station block of docs/design/glow-up-now.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

The full service list to show as absent, and how this shares with the Now page's docked state.
