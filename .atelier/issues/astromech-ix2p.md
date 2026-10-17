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
status: "in_progress"
title: "EDSM split: two poisoned events hold the queue; cap re-halves from 50"
updated_at: "2026-10-17T22:26:23.903779752+00:00"
---

## Goal

EDSM batch splitting copes with several poisoned events and does not re-halve a known-poisoned event from a full batch every time.

## Done when

- A second event that fails alone in the same pass no longer holds the whole queue for up to an hour (today the outage guard treats two single failures in a pass as an outage and calls hold_due with the second event's backoff).
- The cap doubling after each success (Courier.cap in crates/astromech-eddn/src/edsm/mod.rs) does not send a poisoned event back through 50, 25, 13, ... halving on every pass; a test with two poisoned events shows the good events still go promptly.
Found in review of astromech-5llf (findings 3 and 5).
