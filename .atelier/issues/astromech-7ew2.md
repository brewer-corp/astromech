---
created_at: "2026-11-04T21:29:55.819837135+00:00"
id: "astromech-7ew2"
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
  - kind: "review"
    id: "astromech-wesu"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDSM drain: split failed refused batches; edge-case tests"
updated_at: "2026-11-04T21:29:55.819837135+00:00"
---

## Goal

An EDSM backlog that holds one event EDSM always refuses still drains in batches after an outage, and the drain and rate-limit tests pin the edge cases.

## Done when

- A failed batch of previously refused events is split in halves (as decision astromech-sklz describes) instead of dropping the rest of the pass to one event per request; a drain test with one poisoned event in a 201-event backlog takes far fewer requests than events (R1, astromech-wesu).
- A table test covers X-Rate-Limit headers that are missing, unreadable, negative, huge or a Unix timestamp (R2, astromech-ys8x).
- The outage drain test asserts every event was stored exactly once, not only that the queue is empty (R3, astromech-kusu).

## Context

Deferred review findings on task/astromech-az6g: crates/astromech-eddn/src/edsm/mod.rs:509-514, edsm/tests.rs:715 and :725.
