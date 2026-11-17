---
schema: atelier.item
schema_version: 3
id: astromech-evj3
title: Exploration > Biology as a card per body
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:40.418245755Z
---

## Goal

Each body with biology is a card: its genera with values, sample progress as pips (1 of 3), and while sampling a gauge of distance against the colony range. It replaces two tables.

## Done when

- [ ] (provisional) The page's files leave `TOKENS_EXEMPT` in `app/eslint.config.js` (astromech-4wi1): the rebuilt page uses only tokens and role classes for colour, font size and spacing.
- [ ] (provisional) The sampling state and both body lists render as cards from the existing replies.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Colours, selection, result rows, text size and the tabs follow DESIGN.md revision 5 (astromech-n41x); captures are taken at 1100 x 700 at 100 percent and 2560 x 1440 at 145 percent. Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc).  Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

Whether 'reported by other players' merges into the same cards or stays a second group.
