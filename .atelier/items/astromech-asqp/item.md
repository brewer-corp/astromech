---
schema: atelier.item
schema_version: 3
id: astromech-asqp
title: 'Glance test: Materials has no one main value, only a grid of 28 counts'
status: draft
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T15:07:41.813147149Z
---

## Goal

On Materials, both glance-test readers (astromech-crxa, 17 Nov 2026) found no single main value among the grid's counts. DESIGN.md's Board gives every tile its own main value, and the Raw grid has 28, so the page has no one main value, which the mission's glance-test criterion asks of every page. Either the page gets one (the kind's total held against capacity, say, now a small Label line) or Sumeet takes the grid out of that part of the criterion.

## Done when

- [ ] The lead of astromech-xajc, or Sumeet, decides between changing the design and taking this page state out of the glance-test criterion, and the decision is recorded (DESIGN.md or the page's guide for a design change, the mission's `notes.md` for a scope change).
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "main value" as the sheet's row 10 expects; the answers and mark are added to `glance-test.md`.

## Scope

Materials, in `app/src/modules/materials/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 10, and "What failed and why"). The capture: `docs/design/final/materials-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
