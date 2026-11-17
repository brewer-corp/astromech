---
schema: atelier.item
schema_version: 3
id: astromech-ng71
title: 'Glance test: Ships with a module selected reads the head''s jump range as the main value, not the planned grade'
status: approved
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
- [x] The page is recaptured at 2560 x 1440 at 145 percent the way the glance test takes its captures, and a fresh agent given only that capture and the glance test's prompt answers 'main value' as the answer sheet's row for this page expects; the readers' answers and marks are recorded beside the captures in `docs/design/glance-ng71/README.md`. astromech-crxa copies them into `docs/design/final/glance-test.md` when it resumes.

## Scope

Ships with a module selected, in `app/src/modules/engineering/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 7, and "What failed and why"). The capture: `docs/design/final/ships-module-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.

## Report

**Ships with a module selected now passes the glance test.** Both fresh readers named the planned grade, G5, as the main value. In crxa's rounds 1 and 2, readers had picked the head's 32.9 ly jump range. The fix is that the head's four stats draw in Value L, Options Grey, so the panel's G5 is the page's one Value XL. Changing size and colour was enough here; nothing had to move. The second reader picked G5 without hedging.

**Open question for the lead: the new row 7a (nothing selected) fails on can change, with both readers.** The sheet had no unselected-Ships row, so I wrote one and committed it (a8203ebe) before any reader saw the capture. Its main value, the ship's name, passed with both readers. Its can change asks for picking a module row. Neither reader named it; both named the Loadout/Plan choice and the fleet strip. With no panel open, the loadout rows give no sign they can be picked. This is a separate question from ng71's. Two answers work: accept the fleet strip in row 7a, or give the unselected loadout a cue. It blocks crxa's criterion until decided.

The change:

- `app/src/modules/engineering/View.svelte`: the head's stats use `.type-value-l` with no `.main`, whether or not a module is selected.
- The stats block is shared by Loadout and Plan, so Plan changes as well. Plan's jump range used to be Orange Lit; it now has no orange main value in the head. I kept one rule for both views instead of branching on `view`, because the new DESIGN.md line applies everywhere. The Plan view is captured too.
- DESIGN.md, Typography > Hierarchy: the Value L row now says "each of a row of equal stats". A paragraph under the table says such a row is Value L, Options Grey and never the main value, and gives the glance test as the reason.

Evidence:

- `docs/design/glance-ng71/` holds the before and after captures: module selected, nothing selected and Plan, at 2560 x 1440 at 145 percent and 1100 x 700. The plans that take them are there too, along with `README.md`, which has the two sheet rows, all four readers' answers verbatim and the marks.
- All twelve captures measure 0 for sideways, clipped, overlaps, small, contrast and far.
- `before/ships-module-2560x1440.png` is byte for byte the capture crxa's rounds used.
- astromech-crxa copies the README's rows into `glance-test.md` when it resumes, as the amended Done when line says. An earlier commit on this branch copied crxa's glance files across; a later commit removes them, so the branch leaves `docs/design/final/` as main has it.
- `pnpm check` reports 0 errors and 0 warnings, `pnpm lint` is clean, and `pnpm test` passes 100 files and 922 tests.
- No test changed. The hero test reads values, not classes, and a role swap is a visual change, so captures cover it, not a test.
