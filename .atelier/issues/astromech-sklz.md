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
status: "todo"
title: "EDSM: should server errors ever give up a batch, and should refused batches be split?"
updated_at: "2026-10-17T05:38:36.480571994+00:00"
---

## Context

Exempting outages protects against losing a long outage's events; splitting costs more requests against EDSM's rate limits.

## Question

After astromech-ky3d, HTTP 5xx, 408, 429 and request-level 5xx answers from EDSM are held and retried forever, so a batch EDSM always fails with a 500 (one bad event) blocks the queue; and a refused batch is given up whole rather than split, losing the good events with the bad one (review findings astromech-oixz and astromech-ugdy).

## Options

- Keep: outages never drop data; a poisoned batch needs a manual purge.
- Give up 5xx after a long limit (e.g. a day of tries).
- Split a refused or repeatedly failing batch in halves down to single events before giving up.
