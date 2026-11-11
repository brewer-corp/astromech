---
schema: atelier.item
schema_version: 3
id: astromech-5s9h
title: 'Engineering build plan: link to the Faction states finder instead of embedding Nearby'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T15:11:03.296420676Z
trivial: null
caused_by: null
created_at: 2026-11-05T15:11:03.296420676Z
---

## Goal

The engineering build plan sends the commander to Places for "where is X", like Materials does (decision astromech-g8d5, item 5), instead of embedding its own lookup list.

## Done when

- [ ] - `modules/engineering/Build.svelte` (around lines 425 and 451) links into `#/places/faction-states/<query>` with the input filled instead of embedding `<Nearby>`; `Nearby.svelte` is removed if nothing else uses it. - A component test follows the link; `pnpm lint && pnpm check && pnpm build && pnpm test` pass; a `scripts/ui` screenshot of the build plan.

## Context

Deferred finding astromech-tc9s from the kwoi review.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-kwoi (Move the Trade and Materials finders into Places)
