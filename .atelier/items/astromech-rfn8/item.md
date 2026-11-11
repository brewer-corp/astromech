---
schema: atelier.item
schema_version: 3
id: astromech-rfn8
title: 'Lookup rows: leave out a text column whose cell is the same on every row'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T08:26:02.937067106Z
trivial: null
caused_by: null
created_at: 2026-11-05T08:26:02.937067106Z
---

## Goal

Text fact columns keep useful width at 1100 px: a column that says the same thing on every row (e.g. Outfitting's Module once class and rating are chosen) is dropped and its value moves to the constraint summary.

## Done when

- [ ] - SearchKit omits a text fact column whose cell is identical on every shown row, and names that value in the summary line; a test covers it. - A `scripts/ui` shot of Outfitting at 1100 px with class and rating chosen shows Faction wider than the ~40 px it gets today. - `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred finding astromech-a400 from the jp36 review (reviewer-sonnet-jp36), which decided against fixed widths (they push the facts out) and a width floor (it triggers the compact row).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-jp36 (Lookup result row: main value first, aligned columns, DESIGN.md states)
