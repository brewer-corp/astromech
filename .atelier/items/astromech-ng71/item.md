---
schema: atelier.item
schema_version: 3
id: astromech-ng71
title: 'Glance test: Ships with a module selected reads the head''s jump range as the main value, not the planned grade'
status: draft
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: astromech-8wr4
created_at: 2026-11-17T15:07:41.760464010Z
---

## Goal

On Ships with a module selected, the glance-test readers (astromech-crxa, 17 Nov 2026) did not find the planned grade, G5, as the main value: round 1 saw it compete with the head's 32.9 ly jump range, and round 2 picked the jump range. astromech-8wr4 keeps the head's four hero values in Value XL (Options Grey once a module is selected) and moves the main value to the panel's planned grade, so the head's figures sit higher and as large.

## Done when

- [ ] The lead of astromech-xajc, or Sumeet, decides between changing the design and taking this page state out of the glance-test criterion, and the decision is recorded (DESIGN.md or the page's guide for a design change, the mission's `notes.md` for a scope change).
- [ ] With a design change: the page is recaptured at 2560 x 1440 at 145 percent the way `docs/design/final/glance-plan.json` takes its captures, and a fresh agent given only that capture and the prompt in `docs/design/final/glance-test.md` answers "main value" as the sheet's row 7 expects; the answers and mark are added to `glance-test.md`.

## Scope

Ships with a module selected, in `app/src/modules/engineering/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 7, and "What failed and why"). The capture: `docs/design/final/ships-module-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.
