---
created_at: "2026-10-17T05:38:36.480571994+00:00"
id: "astromech-sklz"
issue_type: "decision"
labels: []
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
closed_at: "2026-10-17T16:23:46.793285976+00:00"
status: "done"
title: "EDSM: should server errors ever give up a batch, and should refused batches be split?"
updated_at: "2026-10-17T16:23:46.793285976+00:00"
---

## Context

Exempting outages protects against losing a long outage's events; splitting costs more requests against EDSM's rate limits.

## Question

After astromech-ky3d, HTTP 5xx, 408, 429 and request-level 5xx answers from EDSM are held and retried forever, so a batch EDSM always fails with a 500 (one bad event) blocks the queue; and a refused batch is given up whole rather than split, losing the good events with the bad one (review findings astromech-oixz and astromech-ugdy).

## Options

- Keep: outages never drop data; a poisoned batch needs a manual purge.
- Give up 5xx after a long limit (e.g. a day of tries).
- Split a refused or repeatedly failing batch in halves down to single events before giving up.

## Decision

Send as much as possible, best effort (Sumeet, 17 Oct 2026). EDSM already answers per event when a request goes through, and those answers are used as they are. When a whole request fails without per-event answers (HTTP 5xx, a request-level 5xx code, 413 or an unreadable answer), split the batch in halves on the next tries, down to single events, so the good events go and only the one EDSM chokes on is lost. Request-level 5xx no longer holds forever: like a refusal, it is given up after the attempt limit, counted per event once the batch is down to one. Upload is best effort, so no extra guarantees beyond that.

## Close Reason

Split failing batches down to single events; give up request-level 5xx after the attempt limit (Sumeet, 17 Oct)
