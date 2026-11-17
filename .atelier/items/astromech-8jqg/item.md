---
schema: atelier.item
schema_version: 3
id: astromech-8jqg
title: 'Glance test: Now in space has two main values, the orange mining rate and the larger cargo total'
status: in_review
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

- [x] Decided by the lead of astromech-xajc on 19 Nov 2026: change the design. On the Now page only the block that holds the page's main value draws its value in Value XL, Orange Lit; every other block's value steps down to Value L in Options Grey, in every state (docked, in space, not running), so size and colour agree on which one leads. DESIGN.md's Board says a Board with one page-level main value steps the others down. The change is built so, with before-and-after captures at 2560 x 1440 and 1100 x 700, and `pnpm check`, `pnpm lint` and `pnpm test` pass.
- [x] The page is recaptured at 2560 x 1440 at 145 percent the way the glance test takes its captures, and a fresh agent given only that capture and the glance test's prompt answers 'main value' as the answer sheet's row for this page expects; the readers' answers and marks are recorded beside the captures in `docs/design/glance-8jqg/README.md`. astromech-crxa copies them into `docs/design/final/glance-test.md` when it resumes.

## Scope

Now in space, in `app/src/modules/now/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 2, and "What failed and why"). The capture: `docs/design/final/glance/now-space-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.

## Report

The decided step-down alone did not pass. Two fresh readers still weighed the first block's white cargo total against the orange mining rate in the fourth column. I added one change: the blocks now stand in the order the main value is picked (Mining, Sell your cargo, This session, Powerplay), so the main value's block leads the row. Two new readers then both named 18 t per hour. The lead of astromech-xajc accepted the reorder on 17 Nov 2026: the block that holds the main value leads in position as well as in size and colour.

- **Step-down, as decided (a7b87ea2).** On Now, only the main value's block draws Value XL, Orange Lit. Every other block's value is Value L, Options Grey, in every state; Powerplay's merits were never main and are always L now. While Sell your cargo asks again, its total keeps XL but drops the orange, as before. DESIGN.md's Board row says a Board with one page-level main value steps the others down.
- **Reorder (f7680b98).** Mining moves from last to first. Docked and not running look the same as before, because the main block was already first there. The order is fixed, not chosen per read, so blocks don't jump when the cargo total arrives. Edge case: during a mining run with no rate yet, Mining shows "–" in Value L first, and the cargo total is the orange XL second. The Board row in DESIGN.md states the order rule.
- **Captures.** `docs/design/glance-8jqg/before/` and `after/` hold docked, in space and not running at 2560 x 1440 at 145 percent and at 1100 x 700, taken with `audit.mjs` and `docs/design/glance-8jqg/plan.json`, built like `glance-plan.json`. The audit finds nothing to flag in any of them (no sideways scroll, clipping, overlaps, small text or contrast faults). `after/` is the final build.
- **Readers.** All four readers' verbatim answers and marks are in `docs/design/glance-8jqg/README.md`. Each reader was a fresh agent given one copied capture and the round 2 prompt, cut to one screenshot and without the route question. Step-down only: A and B both fail on main value. With the reorder: C and D both pass on main value. C answered "none" for can change and missed the Add stop and Set destination icons. Those icons are unchanged, and that question passed in both of astromech-crxa's rounds.
- **Where the answers live.** `docs/design/final/glance-test.md` exists only on astromech-crxa's unmerged branch, so the lead reworded the second Done when line: the answers and marks are in `docs/design/glance-8jqg/README.md`, and astromech-crxa copies them into row 2 when it resumes.
- **Checks.** `pnpm check`, `pnpm lint` and `pnpm test` (922 tests) all exit 0 on f7680b98.

