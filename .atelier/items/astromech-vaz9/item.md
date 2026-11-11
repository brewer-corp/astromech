---
schema: atelier.item
schema_version: 3
id: astromech-vaz9
title: 'Faction states finder: check FACTION_STATES against the core''s query ids'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T15:11:03.132592413Z
trivial: null
caused_by: null
created_at: 2026-11-05T15:11:03.132592413Z
---

## Goal

The Faction states finder and the Materials links cannot drift from the core's faction-state queries.

## Done when

- [ ] - `FACTION_STATES` in app/src/lib/galaxy.ts is generated from, or checked by a test against, `astromech_galaxy::remote::QUERIES` (ids and order), e.g. a Rust test that writes or compares a fixture the app test reads. - A new Rust id without an app entry fails a test; `pnpm test` and the Rust test pass.

## Context

Deferred finding astromech-abu3 from the kwoi review (reviewer-fable-kwoi). Same shape as astromech-z9rd (hotspot ores); one mechanism could serve both.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-kwoi (Move the Trade and Materials finders into Places)
