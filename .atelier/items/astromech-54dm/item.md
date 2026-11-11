---
schema: atelier.item
schema_version: 3
id: astromech-54dm
title: Gamestate tests for docked Location and FSDJump clearing station info
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T22:18:23.097767574Z
---

## Goal

No test covers Location with Docked:true filling station_info or FSDJump clearing it; landing pads stay blank after logging in docked.

## Done when

- [ ] Gamestate tests cover both paths, and pads after a docked login are either filled or documented as unknown until Docked.

## Context

Deferred from the astromech-zig3 review.
