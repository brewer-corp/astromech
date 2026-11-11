---
schema: atelier.item
schema_version: 3
id: astromech-xes6
title: Pre-render frequent callout words
status: draft
mission: null
labels:
- size-s
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.663929995Z
---

## Goal

Pre-render fixed words of frequent callouts (ore names, "Motherlode", "Cargo scan") so prospector and scan lines start instantly.

## Done when

- [ ] Time from event to first audio for a prospector line drops below the uncached time (measured).

## Context

Extends the existing confirmation cache.
