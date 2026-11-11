---
schema: atelier.item
schema_version: 3
id: astromech-z9rd
title: 'Hotspot ores: one list shared by the app and the core, or a test that they agree'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T09:13:45.620126617Z
trivial: null
caused_by: null
created_at: 2026-11-05T09:13:45.620126617Z
---

## Goal

The Mining hotspots finder's ore suggestions cannot drift from the ores the core searches for.

## Done when

- [ ] - `ORES` in app/src/lib/hotspots.ts is generated from, or checked by a test against, the Rust ore list used by the hotspot search and the voice `{mineral}` slot. - `pnpm test` (and the Rust test if the check lives there) pass.

## Context

Deferred finding astromech-otpp from the f7i0 review.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-51ue (Galaxy rework: route strip, Places finders, one search box)
