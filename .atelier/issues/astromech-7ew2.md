---
created_at: "2026-11-04T21:29:55.819837135+00:00"
id: "astromech-7ew2"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-7ew2
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
closed_at: "2026-11-04T22:18:45.963573507+00:00"
status: "done"
title: "EDSM drain: split failed refused batches; edge-case tests"
updated_at: "2026-11-04T22:18:45.963573507+00:00"
---

## Goal

An EDSM backlog that holds one event EDSM always refuses still drains in batches after an outage, and the drain and rate-limit tests pin the edge cases.

## Done when

- A failed batch of previously refused events is split in halves (as decision astromech-sklz describes) instead of dropping the rest of the pass to one event per request; a drain test with one poisoned event in a 201-event backlog takes far fewer requests than events (R1, astromech-wesu).
- A table test covers X-Rate-Limit headers that are missing, unreadable, negative, huge or a Unix timestamp (R2, astromech-ys8x).
- The outage drain test asserts every event was stored exactly once, not only that the queue is empty (R3, astromech-kusu).

## Context

Deferred review findings on task/astromech-az6g: crates/astromech-eddn/src/edsm/mod.rs:509-514, edsm/tests.rs:715 and :725.

## Notes

- 2026-11-04 [decision] A refused batch (refused events batched after a store in the same pass) that fails as a whole is halved in the same pass, not on a later pass: a pass-local cap for refused batches is set to ceil(n/2) on such a failure and doubled (to 50 at most) after each request that stores something. Batch failures still count no try; the event that ends up alone counts a refusal as before.
  Reason: Within the pass keeps astromech-dxv1 (good events between known-poisoned ones all go in the same pass) and needs no new stored state; EDSM has just stored something, so an immediate retry is not against an outage. Doubling on store rather than resetting to 50 after the bad event is isolated keeps a mid-pass outage to about log2(50)+3 requests before the outage guard stops the pass. Simulated 60-min 503 outage, 201 events, one poisoned at index 100: 77 recovery requests before, 29 after (23 for the drain, 6 the poisoned event alone in later passes until given up); also checked with the poison at 0, 1, 50 and 199.
  Rejected: Halving across passes with a held queue, as for never-refused batches: one poisoned event would hold the outage backlog a minute per split.
  Rejected: Reset the cap to 50 once the bad event fails alone: cheaper with one poison, but a mid-pass outage costs about 7 requests per isolation.

## Close Reason

Implementer report (commit 854f6b39). Changed: crates/astromech-eddn/src/edsm/mod.rs deliver(): the pass-local flag refused_in_batches is replaced by refused_cap (starts at 50); when a refused batch fails as a whole (Refused, n > 1), refused_cap = ceil(n/2) and the next request in the same pass takes that many refused events, down to one, whose refusal counts as before; refused_cap doubles (max 50) after each request that stores something. Batch failures still count no try. Module doc and docs/guide/data-sharing.md say so. Tests (edsm/tests.rs): drain_after_an_outage helper shared by after_an_hour_of_503s_the_queue_drains_in_batches (now asserts all 201 events stored exactly once, R3) and new a_poisoned_event_in_an_outages_backlog_still_drains_in_batches (R1: one Poison at index 100 in the 201-event backlog, recovery EDSM answers 500 for any request holding it; asserts every other event stored once, the poison given up, requests*5 <= events); new edsms_rate_limit_headers_wait_at_most_an_hour (R2: Remaining missing, 'lots', -1, 5 -> no wait; Reset ' 600 ' -> 600, missing/'soon'/-30/20-digit overflow -> 60, 86400 and Unix timestamp 1800000600 -> 3600; transport error -> no wait). Proof: poisoned drain 77 recovery requests before (test failed: '77 requests for 201 events'), 29 after (23 drain + 6 solo retries of the poison until given up); the test also passes (requests*5 <= events) with the poison at 0, 1, 50 or 199 (checked ad hoc, not committed; counts not recorded). No-poison drain unchanged at 10. Removing the 3600 s cap makes the table test fail. cargo test -p astromech-eddn 153+2 pass incl. the dxv1 tests, clippy -D warnings and fmt clean, check-guide.py 0 problems. Choices: see the decision note (halve in the same pass; double on store instead of resetting to 50). Deviations: none. Surprises: before the fix the poisoned case cost 77, not 168, since only the passes in which the poison was due fell back to singles. A 20-digit Reset fails u64 parsing and waits 60 s rather than 3600 s (pinned by the test). Open items: independent review outstanding.
