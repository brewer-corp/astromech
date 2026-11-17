---
schema: atelier.item
schema_version: 3
id: astromech-59n6
title: 'Grade hexagons: a planned grade cannot be told from a fitted one'
status: approved
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by:
- astromech-m6gw
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Aa8ef4a65-25d6-4ee1-a4a2-8816ddbf5ba6
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

- [x] DESIGN.md's "Engineer rank and blueprint grade" row gives states that tell apart at 16 px without comparing two oranges: what is reached (a rank held, a grade fitted) White; what a plan adds, Orange; a grade an engineer offers that is neither, and an unreached one, by a mark that differs in shape or fill as well as colour (for example an outline against a solid). The row says which and why, and the non-colour cue stays the count plus the accessible name.
- [x] `Hexes.svelte` draws those states, and every use follows from the component, not from page CSS: Ships > Loadout rows and panel, Ships > Suits rows, panel and loadout strip, the Engineers search and the Engineers tree.
- [x] Captures of one row with fitted 3 and planned 5 beside a row with fitted 5, at 100 and 145 percent, before and after, in `docs/design/`; the Ships, Suits and Engineers captures that show hexagons are retaken.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass.

## Scope

The hexagon's states and DESIGN.md's row for them. No page layout changes.

## Report

A planned grade now reads apart from a fitted one at row size. On Ships > Loadout the Frame Shift Drive (fitted 3, planned 5) draws three White hexagons and two Orange ones, beside the Power Plant's five White ones: [before](../../../docs/design/hex-states/before-loadout-1100x700.png), [after](../../../docs/design/hex-states/after-loadout-1100x700.png), and the same at 145 percent (`*-2560x1440.png`).

**The states** (DESIGN.md's "Engineer rank and blueprint grade" row, and `Hexes.svelte`):
- Reached: a rank held, a grade fitted, and the grade a held rank gets today. The full mark, White.
- What a plan adds: the full mark, Orange.
- A grade an engineer offers that is neither reached nor planned: the mark's ring alone, without its centre, Label Grey. It comes from a new file, `app/public/engineers/access-ring.svg`, which is `access.svg` with the centre path removed. NOTICE.md credits it under the same licence.
- Unreached: the full mark, Mark Off.

Offered and unreached differ by fill (ring against full mark) as well as by colour. The non-colour cue is still the count plus the accessible name. I counted the grade a held rank gets today as reached, because the held rank is what reaches it. So in the Engineers search, "Grade 2 today, reaches 5" draws two White marks and three rings. Before this change, a locked engineer's five Orange Mid marks looked the same as a rank-5 engineer's five Orange ones ([before](../../../docs/design/hex-states/before-search-2560x1440.png), [after](../../../docs/design/hex-states/after-search-2560x1440.png)).

**The component:** `tone` is gone. A page passes counts (`n`, and either `plan` or `offers`) and never a colour, and no page CSS reaches into `.hex`. Uses:
- Loadout rows and Suits rows and panel pass `plan`.
- The Engineers search and the Engineers panel's Works on pass `offers`.
- ModulePanel, the suit loadout strip and the Engineers tree pass `n` only.

**Captures:**
- `docs/design/hex-states/` holds before and after at 1100x700 (100 percent) and 2560x1440 (145 percent) for Loadout, Suits, the Engineers panel and the search, with `before.json`, `after.json` and both reports. The before set was served from an unchanged checkout of main.
- The `ship-loadout`, `suits` and `engineers` plans were retaken. That includes `engineers-search-after`. The older `engineer-search/` shots have no plan and were not retaken; the `hex-states` search shots cover that page.

**Two faults outside this item:**
- `pnpm test` failed on main. `pageSizing.test.js` still named `station/View.svelte` as a page not yet enrolled, but astromech-7vxi enrolled it. Commit 58db0e65 swaps it for `logbook/View.svelte`, which is not enrolled.
- Not fixed, and needs filing: the `ship-loadout` plan's `no-fleet` shot times out after 30 s on main too, so `audit.mjs` stops before writing `report.json`. I retook the other 18 shots from a temporary plan and kept the old `no-fleet` entry in the report. Its PNG has no hexagons and did not change.

`pnpm check`, `pnpm lint` and `pnpm test` (885 tests) pass.

### Responses

1-5f6q.1: fixed in f9e327dd
