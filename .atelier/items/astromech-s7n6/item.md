---
schema: atelier.item
schema_version: 3
id: astromech-s7n6
title: 'Glance test: Engineers searched for a module reads ''None unlocked yet'' as what needs the commander'
status: draft
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: astromech-7did
created_at: 2026-11-17T15:07:41.866538740Z
---

## Goal

On Engineers searched for "laser" with no engineer unlocked, both glance-test readers (astromech-crxa, 17 Nov 2026) named the "None unlocked yet: nearest to unlock" line as what needs the commander. astromech-7did makes the nearest engineer to unlock and its next step the main answer and names nothing separately as needing the commander, so the sheet accepted the next step or none. The page needs one thing the commander is to act on, or the sheet needs to say the none-unlocked state is it.

## Done when

- [ ] The lead of astromech-xajc, or Sumeet, decides between changing the design and taking this page state out of the glance-test criterion, and the decision is recorded (DESIGN.md or the page's guide for a design change, the mission's `notes.md` for a scope change).
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "needs the commander" as the sheet's row 12 expects; the answers and mark are added to `glance-test.md`.

## Scope

Engineers searched for "laser", in `app/src/modules/unlocks/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 12, and "What failed and why"). The capture: `docs/design/final/glance/engineers-laser-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
