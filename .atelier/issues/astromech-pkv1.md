---
created_at: "2026-10-17T02:45:47.141212496+00:00"
id: "astromech-pkv1"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Market comparison: handle truncated nearby answers for carried goods"
updated_at: "2026-10-17T02:45:47.141212496+00:00"
---

## Goal

Make the Station market comparison handle a truncated nearby answer for carried goods.

## Done when

- `compare_here` (crates/astromech-market/src/here.rs) handles `Prices.truncated` for carried lines, the way the trade cargo plan does with its second request, so a market that can take the whole load is never cut before ranking.
- A test covers a truncated answer.
- From the review of astromech-qw2m, finding R3 (low).
