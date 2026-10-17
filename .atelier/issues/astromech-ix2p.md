---
created_at: "2026-10-17T17:15:12.541037081+00:00"
id: "astromech-ix2p"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-ix2p
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T22:37:53.020455709+00:00"
status: "done"
title: "EDSM split: two poisoned events hold the queue; cap re-halves from 50"
updated_at: "2026-10-17T22:37:53.020455709+00:00"
---

## Goal

EDSM batch splitting copes with several poisoned events and does not re-halve a known-poisoned event from a full batch every time.

## Done when

- A second event that fails alone in the same pass no longer holds the whole queue for up to an hour (today the outage guard treats two single failures in a pass as an outage and calls hold_due with the second event's backoff).
- The cap doubling after each success (Courier.cap in crates/astromech-eddn/src/edsm/mod.rs) does not send a poisoned event back through 50, 25, 13, ... halving on every pass; a test with two poisoned events shows the good events still go promptly.
Found in review of astromech-5llf (findings 3 and 5).

## Notes

- 2026-10-17 [note] Limit: the outage guard now stops a pass after three events fail alone in a row with nothing stored in between (was two). Three or more poisoned events that are all due in the same pass still stop it, for the last one's back-off. Tried counting only never-refused events, but that lets a sustained outage burn the whole queue (every event of the 5xx outage test gave up), so the simple rule stays; the outage test's bound moved from 2..4 to 2..6 events given up. A review noted a reset of the counter only when something was stored (done).

## Close Reason

A second event failing alone no longer holds the queue (the pass stops after three in a row with nothing stored; a stored event resets the count), and an event that already failed alone is sent by itself, not back through the halving. Tests two_poisoned_events_do_not_hold_the_queue and a_known_poisoned_event_is_not_sent_back_through_the_halving failed before. Limit: three or more poisoned events due together still stop the pass; outage test bound widened to 2..6. Review recorded.
