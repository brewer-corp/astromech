---
schema: atelier.item
schema_version: 3
id: astromech-cu7d
title: Validate the linked-places increment on the Linux build
status: ready
mission: astromech-7q0z
labels:
- size-s
- validation
priority: high
blocked_by:
- astromech-sgwi
- astromech-wla4
- astromech-xlbz
implementers: []
ready:
  by: conversion
  at: 2026-11-05T06:27:50.754459448Z
trivial: null
caused_by: null
created_at: 2026-11-05T06:27:50.754459448Z
---

## Goal

Check the linked-places increment's outcome independently on the Linux build, against astromech-os70's Done when.

## Done when

- [ ] Each claim in astromech-os70's Done when has an explicit result and a linked evidence receipt (recorded-response test runs and `scripts/ui` screenshots), and the handoff states failures, blocked checks and proof limits.

## Context

Spike astromech-yn6y (budget, recipes), decision astromech-bi94, DESIGN.md Lookup result row and Places home.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-os70 (Linked places: engine, recipes, Find near this, Find along route)
