---
schema: atelier.item
schema_version: 3
id: astromech-eq5k
title: Suit engineers in one place; tab-row test excludes by role
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T18:52:09.012627369Z
trivial: null
caused_by: null
created_at: 2026-11-05T18:52:09.012627369Z
---

## Goal

One home for suit engineers, and a tab-row check that can't be fooled by markup.

## Done when

- [ ] - The per-engineer mod list from On foot › Engineers (`app/src/modules/odyssey/View.svelte` around line 356) is on the Engineers page's Suit engineers view, and On foot's Engineers view is dropped or becomes a link to `#/unlocks?kind=suit`; old `#/odyssey?view=engineers` links still land (finding astromech-pkd1). - `app/src/tabrows.test.ts` excludes the Ships rail by what it is (its `nav[aria-label]`) and the Presets grid by its role, not by "has a picture" or "classes differ", so a future tab row with glyphs is still counted (finding astromech-w6gh). - `pnpm lint && pnpm check && pnpm build && pnpm test` pass; `scripts/ui` shots of On foot and Engineers.

## Context

Deferred findings from the sllo review (reviewer-fable-sllo).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-sllo (Two tab rows at most: fold third-level tabs into choices or sections)
