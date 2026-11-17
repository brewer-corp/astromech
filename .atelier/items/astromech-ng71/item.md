---
schema: atelier.item
schema_version: 3
id: astromech-ng71
title: 'Glance test: Ships with a module selected reads the head''s jump range as the main value, not the planned grade'
status: building
mission: astromech-xajc
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A4b5e4767-a3f7-4c82-90a8-fedc5b92af7d%3Adelegate-task%3Aef0a33e5-44e5-4852-9a46-bb9ddb94e10f
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T15:10:56.301215159Z
trivial: null
caused_by: astromech-8wr4
created_at: 2026-11-17T15:07:41.760464010Z
---

## Goal

On Ships with a module selected, the glance-test readers (astromech-crxa, 17 Nov 2026) did not find the planned grade, G5, as the main value: round 1 saw it compete with the head's 32.9 ly jump range, and round 2 picked the jump range. astromech-8wr4 keeps the head's four hero values in Value XL (Options Grey once a module is selected) and moves the main value to the panel's planned grade, so the head's figures sit higher and as large.

## Done when

- [x] Decided by the lead of astromech-xajc on 19 Nov 2026: change the design. On Ships > Loadout the head's four stats are a row of equal facts, not the page's main value: they draw in Value L, Options Grey, with or without a module selected. With a module selected, the panel's planned grade is the one Value XL, Orange Lit on the page. With none selected the page has no Value XL and its main value is the ship's name in Title; the answer sheet's row for the unselected state is checked against that and asked again. DESIGN.md's Hierarchy or Board section says a row of equal stats is Value L. The change is built so, with before-and-after captures at 2560 x 1440 and 1100 x 700, and `pnpm check`, `pnpm lint` and `pnpm test` pass.
- [x] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "main value" as the sheet's row 7 expects; the answers and mark are added to `glance-test.md`.

## Scope

Ships with a module selected, in `app/src/modules/engineering/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 7, and "What failed and why"). The capture: `docs/design/final/ships-module-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.

## Report

**Ships with a module selected now passes the glance test: a fresh reader picked the planned grade, G5, as the main value, where rounds 1 and 2 picked the head's 32.9 ly jump range.** The head's four stats draw in Value L, Options Grey, so the panel's G5 is the page's one Value XL.

**Reconcile first.** `docs/design/final/glance-test.md`, `glance-plan.json` and `glance/` exist only on astromech-crxa's unmerged branch. Commit 5357f0ee brings them over byte for byte from a87aaa08, and this item only appends to `glance-test.md` (a "Round 3" section at the end). Whichever of crxa and this item merges second meets the other's lines in the same file. astromech-vru4 and astromech-8jqg will need the same file and face the same choice.

The change:

- `app/src/modules/engineering/View.svelte`: the head's stats are `.type-value-l` with no `.main`, whether a module is selected or not. With nothing selected the page has no Value XL, and the ship's name in Title leads.
- The stats are one block drawn above both Loadout and Plan, so Plan changes too. Before, Plan's jump range was Orange Lit as the main value. Now Plan with nothing selected has no orange main value in the head. I kept one rule for both views rather than branching on `view`, since DESIGN.md's new line is general. Captured as `ships-plan-*`.
- DESIGN.md, Typography > Hierarchy: Value L's use now includes "each of a row of equal stats", and a paragraph under the table says such a row is Value L, Options Grey and never the main value. The paragraph gives the glance test as the reason.

Captures: the before and after sets are in `docs/design/final/ng71/before/` and `ng71/after/`. Each has Ships with the FSD selected, Ships with nothing selected and Plan, at 2560 x 1440 (145 percent) and 1100 x 700 (100 percent). They come from `docs/design/final/ng71-before.json` and `ng71-after.json` with `scripts/browser/audit.mjs`. All twelve measure 0 on sideways, clipped, overlaps, small, contrast and far. `ng71/before/ships-module-2560x1440.png` is byte for byte the capture rounds 1 and 2 used. `pnpm check` gives 0 errors and 0 warnings, `pnpm lint` is clean, and `pnpm test` passes 100 files and 922 tests. No test changed: the existing hero test reads values, not classes, and a role swap is a visual change, which takes captures, not a test.

Glance test, round 3, recorded verbatim with marks in `glance-test.md`. Each reader was a fresh general-purpose agent given one capture as `01.png` and the round 2 prompt.

- **Row 7, a module selected:** main value G5 ("though Long Way Round and 32.9 ly also compete"), can change the dropdowns and ADD TO PLAN, needs you the "!" materials. All three pass. The reader made one pick and noted the others competing, and round 1 passed rows 25 and 26 on the same pattern.
- **Row 7a, nothing selected:** this row is new. The sheet had no unselected-Ships row, so I added it and committed it (a8203ebe) before any reader ran. Its main value is the lead's: the ship's name. The other two answers come from astromech-8wr4. The reader's main value was "Long Way Round", which passes. Needs you was none, which passes. **Can change fails** as the row is written: the reader named the Loadout/Plan choice and the fleet strip, not picking a module row. With no panel open, nothing on the loadout rows suggests they can be picked. That is outside this item's question (the stats against the main value). The lead decides it: accept the fleet strip in row 7a (picking a ship changes the page, like picking a row) or give the unselected loadout a cue. It blocks astromech-crxa until decided.

