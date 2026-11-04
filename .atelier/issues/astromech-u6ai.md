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
closed_at: "2026-11-04T23:43:55.635467908+00:00"
status: "done"
title: "EDSM drain tests: pin no-try-on-failed-batch and poison positions"
updated_at: "2026-11-04T23:43:55.635467908+00:00"
---

## Goal

The EDSM drain tests pin the rule that a failed batch of previously refused events costs its good events no tries, and the poisoned-event drain holds wherever the poison sits.

## Done when

- A test fails if a failed refused batch (or one of its halves) adds a refusal to its events: only the poisoned event's refusal count grows (R1, astromech-aca0).
- The poisoned-drain test runs with the poison at 0, 49, 50 and 199 (R2, astromech-ycv5).

## Context

Deferred review findings on task/astromech-7ew2: crates/astromech-eddn/src/edsm/mod.rs:513-524, edsm/tests.rs:752-760. Code is correct today; the gap is in the tests.

## Close Reason

Done at 157996b4, tests only; no bug found. New pass-level test a_failed_refused_batch_counts_a_refusal_only_against_the_event_failing_alone (edsm/tests.rs): 40 events refused once, poison at 17, plus one fresh event stored first; one pass halves the refused batch (>=5 multi-event requests), stores all 40 good events, leaves only the poison, whose refusals go 1 -> 2. Mutation check: refuse_at(refusals+1) for each row in the failed refused-batch branch makes it fail (reverted). a_poisoned_event_in_an_outages_backlog_still_drains_in_batches loops the poison over 0, 49, 50, 100 and 199 (100 kept from before); all pass the existing bounds (one stored copy each, poison not stored, requests*5 <= events). New helper refusals_of finds a queued event by its event name. Not covered: several poisons in the drain (reviewer saw 52 and 97 requests, above the 5x bound). Review outstanding (orchestrator).
