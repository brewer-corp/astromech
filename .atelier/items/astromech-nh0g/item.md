---
schema: atelier.item
schema_version: 3
id: astromech-nh0g
title: 'Preview fixtures: per-stop jumps on the neutron and local routes, and lookups near the route scenario''s commander'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T07:57:02.360015345Z
trivial: null
caused_by: null
created_at: 2026-11-05T07:57:02.360015345Z
---

## Goal

The browser preview (`?core=...`) shows the route strip and lookups as the real app would, so screenshots taken from it are honest layout evidence.

## Done when

- [ ] - `app/src/lib/fixtures/plot.json`'s `neutron` and `local` routes carry per-stop `jumps` matching their totals (finding astromech-9ju1), so the preview's Plot and follow shows "N jumps left" as the real plotter does. - With `?core=ready&route=exact` the lookups' origin follows the scenario's commander system rather than Sol (finding astromech-7464). - `pnpm test` passes; a `scripts/ui` shot of Services with `?route=exact` shows the right origin.

## Context

Deferred findings from the 1d92 review (reviewer-fable-1d92). Preview only; the real app is unaffected.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-1d92 (Route strip on every page)
