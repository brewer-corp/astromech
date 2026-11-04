---
created_at: "2026-11-04T23:18:14.315248113+00:00"
id: "astromech-u6ai"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDSM drain tests: pin no-try-on-failed-batch and poison positions"
updated_at: "2026-11-04T23:37:09.229208589+00:00"
---

## Goal

The EDSM drain tests pin the rule that a failed batch of previously refused events costs its good events no tries, and the poisoned-event drain holds wherever the poison sits.

## Done when

- A test fails if a failed refused batch (or one of its halves) adds a refusal to its events: only the poisoned event's refusal count grows (R1, astromech-aca0).
- The poisoned-drain test runs with the poison at 0, 49, 50 and 199 (R2, astromech-ycv5).

## Context

Deferred review findings on task/astromech-7ew2: crates/astromech-eddn/src/edsm/mod.rs:513-524, edsm/tests.rs:752-760. Code is correct today; the gap is in the tests.
