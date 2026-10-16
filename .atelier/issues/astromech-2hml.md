---
created_at: "2026-10-16T20:56:01.123473949+00:00"
id: "astromech-2hml"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-2hml
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T22:10:54.279626863+00:00"
status: "done"
title: "Flaky test: gear_down confirmation latency floor"
updated_at: "2026-10-16T22:10:54.279626863+00:00"
---

## Goal

`crates/astromech-core/tests/gear_down.rs` `confirmation_is_logged_with_latency` asserts a latency floor (550 ms) that a loaded machine misses by a millisecond (measured 549 ms), failing unrelated branches.

## Done when

The test asserts the latency is logged and plausible without a tight wall-clock floor (or uses an injected clock), and passes 20 runs in a row under load.

## Context

Seen twice on feature/astromech-zf3s with no related changes (2026-10-16).

## Close Reason

Test no longer asserts a tight wall-clock floor; 20/20 runs under load
