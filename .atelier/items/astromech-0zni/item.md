---
schema: atelier.item
schema_version: 3
id: astromech-0zni
title: Optional EDDN listener for fresher prices
status: draft
mission: null
labels:
- size-l
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.652294682Z
---

## Goal

Subscribe to EDDN (ZeroMQ `tcp://eddn.edcd.io:9500`) and keep a local market cache around the commander's bubble.

## Done when

- [ ] With the listener on, a price change on EDDN shows in the trade lookup within a minute.

## Context

Only if Ardent's data age turns out to be a problem.
