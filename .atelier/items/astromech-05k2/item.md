---
schema: atelier.item
schema_version: 3
id: astromech-05k2
title: Measure the MVP criteria as a set
status: ready
mission: astromech-610m
labels:
- alpha
- size-m
- validation
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-18T00:41:36.599067839Z
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.656811183Z
---

## Goal

Run and report together: 200 mock-game trials per tool, a 4 h soak replay with zero unintended keypresses, kill switch ≤ 50 ms, action start ≤ 400 ms p95 after PTT release on each device class.

## Done when

- [ ] One report with each criterion's result, attached as evidence receipts.

## Context

[ARCHITECTURE §14](docs/ARCHITECTURE.md) Phase 1. Some are covered by tests already; none are reported together.

## Notes

- 2026-10-17 [note] 17 Oct: no longer blocks the alpha release (astromech-5ggk). The game-free criteria are measured on the Windows runner by the MVP harness (astromech-08w8, merged): 200/200 per T1 tool, 0 unintended keypresses in 240 min, kill switch max 1.2 ms, action start ~396 ms p95 before astromech-e0zz. The device-class and real-game parts stay for v1.0 (in the alpha's test script).

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-huab (M5: v1.0 release)
