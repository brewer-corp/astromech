---
schema: atelier.item
schema_version: 3
id: astromech-0132
title: EDSM keyring error as a hard error, controller sheet greys
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T17:21:38.613712758Z
trivial: null
caused_by: null
created_at: 2026-11-05T17:21:38.613712758Z
---

## Goal

Finish DESIGN.md's remaining surface and state rules on the pages dfvp touched.

## Done when

- [ ] - `Edsm.svelte`'s keyring failure is a hard error: red "!", words and a recovery action (finding astromech-uprs). - `scripts/recolour-sheets.py` stops baking the old grey `#2b2d2e` into the controller sheet headers and the sheets are regenerated, with a before/after look (finding astromech-ztlk). - `pnpm lint && pnpm check && pnpm build && pnpm test` pass; `scripts/ui` screenshots of Materials, My controller, Ships shopping list and Data sharing.

## Context

Two parts left on 17 Nov 2026: the count-first material tiles were built by astromech-z5oh, and the filled selected row is replaced by DESIGN.md revision 5's raised row (astromech-3kf5). Deferred findings from the dfvp review (reviewer-fable-dfvp). The filter-fixed columns question from decision astromech-5p9c stays with the Sell and Mining finders.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-dfvp (Cells mean editable: ember cells for every editable value, hull for read-only things)
