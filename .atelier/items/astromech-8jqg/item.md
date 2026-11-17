---
schema: atelier.item
schema_version: 3
id: astromech-8jqg
title: 'Glance test: Now in space has two main values, the orange mining rate and the larger cargo total'
status: draft
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: astromech-coq6
created_at: 2026-11-17T15:07:35.436684308Z
---

## Goal

On Now in space with cargo aboard, a commander can't tell the page's main value at a glance. Both glance-test readers (astromech-crxa, 17 Nov 2026) saw two candidates: the orange 18 t per hour of the Mining block and the larger white 183,674,700 cr of Sell your cargo. astromech-coq6 draws every block's value in Value XL and makes only the first that shows Orange Lit, so in space the cargo total is the same size with more digits. Docked, where the cargo total is the orange one, the page passed.

## Done when

- [ ] The lead of astromech-xajc, or Sumeet, decides between changing the design and taking this page state out of the glance-test criterion, and the decision is recorded (DESIGN.md or the page's guide for a design change, the mission's `notes.md` for a scope change).
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "main value" as the sheet's row 2 expects; the answers and mark are added to `glance-test.md`.

## Scope

Now in space, in `app/src/modules/now/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 2, and "What failed and why"). The capture: `docs/design/final/glance/now-space-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
