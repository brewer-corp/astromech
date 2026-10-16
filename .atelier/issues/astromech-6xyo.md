---
created_at: "2026-10-16T20:53:31.889432194+00:00"
id: "astromech-6xyo"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-tyjz"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Tidy EDDN switch-off and give-up reporting"
updated_at: "2026-10-16T20:58:07.943137638+00:00"
---

## Goal

Switching EDDN off empties the queue at once, and Settings → Data sharing shows only real send failures as the last error.

## Done when

Tests show:
- set_eddn turning sharing off discards the queue straight away (wake the worker, or purge from the core), so turning it back on within the worker's idle wait (up to 300 s) sends nothing queued before (final EDDN review R3);
- the "switched off: N queued discarded" and "outbox full" rows are not reported as Last error, and a full queue writes one row per overflow episode, not one per enqueue (R4);
- set_eddn flips the shared EDDN_SHARING flag (R5).

## Context

Found in the final review of feature/astromech-tyjz (astromech-xvjs, astromech-09s3).

## Notes

- 2026-10-16 [note] Also (review round 8, R1): an event awaiting its companion file when sharing goes off, with the file arriving after sharing is back on, is sent with its original timestamp; apply the live-age check on that path too.
