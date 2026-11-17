---
schema: atelier.item
schema_version: 3
id: astromech-ij3d
title: 'Glance test: System reads Worth mapping and the unsold total as what needs the commander'
status: draft
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: astromech-dpaz
created_at: 2026-11-17T15:07:41.916097664Z
---

## Goal

On System with a body selected, both glance-test readers (astromech-crxa, 17 Nov 2026) named the "Worth mapping" badges or the 38.7 M cr of unsold data as what needs the commander, where the sheet says none. astromech-dpaz names only the unsold-data "!" line, shown when dying would lose data, as what needs the commander. Without that line the page still reads as if mapping or selling is asked of the commander.

## Done when

- [ ] The lead of astromech-xajc, or Sumeet, decides between changing the design and taking this page state out of the glance-test criterion, and the decision is recorded (DESIGN.md or the page's guide for a design change, the mission's `notes.md` for a scope change).
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "needs the commander" as the sheet's row 26 expects; the answers and mark are added to `glance-test.md`.

## Scope

System with a body selected, in `app/src/modules/exploration/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 26, and "What failed and why"). The capture: `docs/design/final/system-selected-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
