---
schema: atelier.item
schema_version: 3
id: astromech-8jqg
title: 'Glance test: Now in space has two main values, the orange mining rate and the larger cargo total'
status: building
mission: astromech-xajc
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A4b5e4767-a3f7-4c82-90a8-fedc5b92af7d%3Adelegate-task%3A5b4b2702-3ac8-4ff5-ab80-03374b298429
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T15:10:56.301215159Z
trivial: null
caused_by: astromech-coq6
created_at: 2026-11-17T15:07:35.436684308Z
---

## Goal

On Now in space with cargo aboard, a commander can't tell the page's main value at a glance. Both glance-test readers (astromech-crxa, 17 Nov 2026) saw two candidates: the orange 18 t per hour of the Mining block and the larger white 183,674,700 cr of Sell your cargo. astromech-coq6 draws every block's value in Value XL and makes only the first that shows Orange Lit, so in space the cargo total is the same size with more digits. Docked, where the cargo total is the orange one, the page passed.

## Done when

- [ ] Decided by the lead of astromech-xajc on 19 Nov 2026: change the design. On the Now page only the block that holds the page's main value draws its value in Value XL, Orange Lit; every other block's value steps down to Value L in Options Grey, in every state (docked, in space, not running), so size and colour agree on which one leads. DESIGN.md's Board says a Board with one page-level main value steps the others down. The change is built so, with before-and-after captures at 2560 x 1440 and 1100 x 700, and `pnpm check`, `pnpm lint` and `pnpm test` pass.
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "main value" as the sheet's row 2 expects; the answers and mark are added to `glance-test.md`.

## Scope

Now in space, in `app/src/modules/now/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 2, and "What failed and why"). The capture: `docs/design/final/glance/now-space-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
