---
created_at: "2026-10-17T16:23:46.898089328+00:00"
id: "astromech-5llf"
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
title: "EDSM: split a failing batch to isolate the bad event"
updated_at: "2026-10-17T16:54:22.202225967+00:00"
---

## Goal

Lose only the event EDSM can't take when a whole upload request fails, as decided in astromech-sklz.

## Done when

- A batch whose request fails with a request-level 5xx or a refusal is split in halves on the next tries, down to single events; halves that pass are sent.
- A single event still failing that way is given up after MAX_ATTEMPTS and logged; the queue behind it moves.
- Tests: a batch with one poisoned event (always HTTP 500) sends every other event and gives up only that one; a 413 batch is split the same way.

## Context

EDSM answers per event when a request goes through, and `edsm/mod.rs` already settles each event by its own code (a per-event 5xx is retried up to MAX_ATTEMPTS, then given up). The gaps are request-level failures, which carry no per-event answer: HTTP 5xx and request-level 5xx codes are held forever (`Answer::Hold`), so one event that makes EDSM fail every time blocks the queue; and `Answer::Refused` (413, unreadable, wrong length) gives up the whole batch of up to BATCH = 50 after MAX_ATTEMPTS. Review findings astromech-oixz and astromech-ugdy. Best effort: transport errors, timeouts, 408 and 429 can keep holding as they do now.
