---
schema: atelier.item
schema_version: 3
id: astromech-fwbh
title: Don't store repeated identical Outfitting.json snapshots
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T22:18:22.689803896Z
---

## Goal

Stop the station book growing by about 100 MB a year from byte-identical outfitting snapshots, which the game writes with a new timestamp on every visit to the outfitting screen.

## Done when

- [ ] A test shows a repeated snapshot with the same items as the station's latest adds no raw row, and ARCHITECTURE notes the expected growth.

## Context

Deferred from the astromech-zig3 review.
