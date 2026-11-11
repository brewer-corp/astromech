---
schema: atelier.item
schema_version: 3
id: astromech-crxa
title: 'Glance test: an independent read of every page at 2560 x 1440'
status: draft
mission: astromech-xajc
labels:
- validation
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:46.190820706Z
---

## Goal

Mission astromech-xajc's glance-test criterion: on every page outside Controls a reader can point to the main value, the thing they can change and the thing that needs them, without reading. A fresh agent that built none of the pages is tested against answers written down first.

## Done when

- [ ] An answer sheet lists, for every page outside Controls, the expected main value, the expected changeable thing and the expected thing that needs the commander in the screenshot's state, taken from the page's job in its issue and docs/guide. A page with nothing to change or nothing needing attention says 'none' for that question, and 'none' is then the right answer.
- [ ] A fresh agent given only the 2560 x 1440 screenshots from the browser preview, not the sheet, answers the three questions per page; each answer is marked pass or fail against the sheet.
- [ ] Every failed page is fixed and asked again with a new agent until it passes. The criterion is met only when every page passes; a page left failing leaves it unmet unless Sumeet takes that page out of scope.
- [ ] The sheet, the answers and the marks are an evidence receipt on astromech-xajc with --criterion glance-test.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Runs after the last screen increment merges.
