---
schema: atelier.item
schema_version: 3
id: astromech-0132
title: Count-first material tiles, filled selected rows, EDSM keyring error, controller sheet greys
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

- [ ] - Material tiles lead with the count in Value XL, with the cap in the column head, per DESIGN.md and docs/design/design-system-materials-after.png (finding astromech-tuy0). - A row a side panel explains (My controller row, Shopping list row, selected binding) is a solid orange fill with On Orange text, designed against the Listening fill on Keybinds (finding astromech-qqws). - `Edsm.svelte`'s keyring failure is a hard error: red "!", words and a recovery action (finding astromech-uprs). - `scripts/recolour-sheets.py` stops baking the old grey `#2b2d2e` into the controller sheet headers and the sheets are regenerated, with a before/after look (finding astromech-ztlk). - `pnpm lint && pnpm check && pnpm build && pnpm test` pass; `scripts/ui` screenshots of Materials, My controller, Ships shopping list and Data sharing.

## Context

Deferred findings from the dfvp review (reviewer-fable-dfvp). The filter-fixed columns question from decision astromech-5p9c stays with the Sell and Mining finders.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-dfvp (Cells mean editable: ember cells for every editable value, hull for read-only things)
