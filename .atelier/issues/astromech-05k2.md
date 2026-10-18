---
created_at: "2026-10-15T20:36:13.656811183+00:00"
id: "astromech-05k2"
issue_type: "validation"
labels:
- "alpha"
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-rh44"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-08w8"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Measure the MVP criteria as a set"
updated_at: "2026-10-18T00:41:36.599067839+00:00"
---

## Goal

Run and report together: 200 mock-game trials per tool, a 4 h soak replay with zero unintended keypresses, kill switch ≤ 50 ms, action start ≤ 400 ms p95 after PTT release on each device class.

## Done when

One report with each criterion's result, attached as evidence receipts.

## Context

[ARCHITECTURE §14](docs/ARCHITECTURE.md) Phase 1. Some are covered by tests already; none are reported together.

## Notes

- 2026-10-17 [note] 17 Oct: no longer blocks the alpha release (astromech-5ggk). The game-free criteria are measured on the Windows runner by the MVP harness (astromech-08w8, merged): 200/200 per T1 tool, 0 unintended keypresses in 240 min, kill switch max 1.2 ms, action start ~396 ms p95 before astromech-e0zz. The device-class and real-game parts stay for v1.0 (in the alpha's test script).
