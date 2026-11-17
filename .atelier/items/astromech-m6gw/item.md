---
schema: atelier.item
schema_version: 3
id: astromech-m6gw
title: Engineering > Engineers as the unlock tree
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
- astromech-7did
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:41.864039859Z
---

## Goal

The engineers read as the tree they are: nodes joined from whoever introduces them, unlocked ones white with rank hexagons, ones the commander can work on now marked, barred ones in the duller red, the next step for each stated with its progress. It replaces 25 text cards that state prerequisites as '<- Liz Ryder'.

## Done when

- [ ] (provisional) The page's files leave `TOKENS_EXEMPT` in `app/eslint.config.js` (astromech-4wi1): the rebuilt page uses only tokens and role classes for colour, font size and spacing.
- [ ] (provisional) The ship and suit engineer trees render side by side from referred_by and refers, under the search box astromech-7did built; selecting an engineer shows system, specialities and steps.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Colours, selection, result rows, text size and the tabs follow DESIGN.md revision 5 (astromech-n41x); captures are taken at 1100 x 700 at 100 percent and 2560 x 1440 at 145 percent. Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockup: docs/design/glow-up-engineers.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

Layout of the Colonia engineers and of engineers with no referrer (the mockup puts Colonia behind a Bubble / Colonia choice). The mockup shows progress counts on a step ('2 of 5 black markets', '12 of 50'); the unlock reply gives only the step's text, so the counts need a source in the journal or the mockup's counts go.
