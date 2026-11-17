---
schema: atelier.item
schema_version: 3
id: astromech-b49e
title: 'Glance test: Now with Elite not running reads its title as what needs the commander'
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
created_at: 2026-11-17T15:07:41.660466030Z
---

## Goal

On Now with Elite not running, both glance-test readers (astromech-crxa, 17 Nov 2026) named the title "Elite isn't running" as what needs the commander, where the sheet says none. astromech-coq6 makes that line the page's Title, in White, and names only exploration data at risk as what needs the commander. Either the title should not read as a call to act, or the sheet is wrong and a game that is off is what needs the commander here; this item settles which.

## Done when

- [ ] The lead of astromech-xajc, or Sumeet, decides between changing the design and taking this page state out of the glance-test criterion, and the decision is recorded (DESIGN.md or the page's guide for a design change, the mission's `notes.md` for a scope change).
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "needs the commander" as the sheet's row 3 expects; the answers and mark are added to `glance-test.md`.

## Scope

Now with Elite not running, in `app/src/modules/now/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 3, and "What failed and why"). The capture: `docs/design/final/glance/now-not-running-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
