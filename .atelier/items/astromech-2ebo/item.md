---
schema: atelier.item
schema_version: 3
id: astromech-2ebo
title: Cache docked-here state instead of re-reading the journal every 5 s
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T22:18:23.052846459Z
---

## Goal

docked_here re-reads and re-parses the last two journal files on each call, and can briefly disagree with the ship from EngineeringCache.

## Done when

- [ ] docked_here reads GameState from the cached journal state, and a test or inspection shows one source for ship and station.

## Context

Deferred from the astromech-zig3 review.
