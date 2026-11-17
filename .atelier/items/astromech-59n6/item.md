---
schema: atelier.item
schema_version: 3
id: astromech-59n6
title: 'Grade hexagons: a planned grade cannot be told from a fitted one'
status: ready
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by:
- astromech-m6gw
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T11:52:10.296235481Z
trivial: null
caused_by: astromech-n41x
created_at: 2026-11-17T11:52:10.234838394Z
---

## Goal

A row of grade hexagons says at a glance how far the item is engineered and how far the plan takes it. On Ships > Loadout a Frame Shift Drive fitted at grade 3 with grade 5 planned draws three hexagons in Orange and two in Orange Mid (`docs/design/ship-loadout/krait-module-2560x1440.png`, the Frame Shift Drive row beside the Power Plant's five fitted ones): at row size the two oranges read as five lit hexagons, so the plan looks done. Seen by the mission lead on 19 Nov 2026 in astromech-8wr4's capture. DESIGN.md's row "Engineer rank and blueprint grade" separates a grade the commander can get (Orange) from a planned grade not reached (Orange Mid) by that same small step.

## Done when

- [ ] DESIGN.md's "Engineer rank and blueprint grade" row gives states that tell apart at 16 px without comparing two oranges: what is reached (a rank held, a grade fitted) White; what a plan adds, Orange; a grade an engineer offers that is neither, and an unreached one, by a mark that differs in shape or fill as well as colour (for example an outline against a solid). The row says which and why, and the non-colour cue stays the count plus the accessible name.
- [ ] `Hexes.svelte` draws those states, and every use follows from the component, not from page CSS: Ships > Loadout rows and panel, Ships > Suits rows, panel and loadout strip, the Engineers search and the Engineers tree.
- [ ] Captures of one row with fitted 3 and planned 5 beside a row with fitted 5, at 100 and 145 percent, before and after, in `docs/design/`; the Ships, Suits and Engineers captures that show hexagons are retaken.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass.

## Scope

The hexagon's states and DESIGN.md's row for them. No page layout changes.
