---
schema: atelier.item
schema_version: 3
id: astromech-vru4
title: 'Glance test: Station''s main value is outshouted by For sale here''s larger ranked profit'
status: ready
mission: astromech-xajc
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T15:10:56.301215159Z
trivial: null
caused_by: astromech-7vxi
created_at: 2026-11-17T15:07:41.712215369Z
---

## Goal

On Station, both glance-test readers (astromech-crxa, 17 Nov 2026) took For sale here's +52,506 cr/t on Tritium as the page's main value, not the hold's +3,921 cr/t on Gold. astromech-7vxi makes the first cargo line's gain the main value in Orange Lit and For sale here's profit the ranked value, also orange, both in Value L, so the larger ranked figure further down outshouts the main value.

## Done when

- [ ] Decided by the lead of astromech-xajc on 19 Nov 2026: change the design. On Station the hold's gain per tonne on the first cargo line is the page's one main value, in Value XL, Orange Lit. The ranked values in For sale here and the lists below keep Orange as ranked values but at row size (the grouped row's value size), so no figure further down is as large as the main value. The Station guide page says what leads. The change is built so, with before-and-after captures at 2560 x 1440 and 1100 x 700, and `pnpm check`, `pnpm lint` and `pnpm test` pass.
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "main value" as the sheet's row 4 expects; the answers and mark are added to `glance-test.md`.

## Scope

Station, docked, in `app/src/modules/station/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 4, and "What failed and why"). The capture: `docs/design/final/station-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
