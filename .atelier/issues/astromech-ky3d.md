---
created_at: "2026-10-17T05:04:37.123409707+00:00"
id: "astromech-ky3d"
issue_type: "task"
labels: []
priority: "P2"
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
title: "EDSM upload: recover after a key fix, give up on refused batches, flight-log timeout"
updated_at: "2026-10-17T05:04:37.123409707+00:00"
---

## Goal

Follow-ups from the astromech-ah0i review:
- astromech-69wx: after EDSM answers 201-203 (wrong name or key), queued events wait up to an hour; saving a corrected key in Settings should make them due again and wake the uploader, and clear the stale error.
- astromech-3pkl: a request EDSM refuses (any non-200 status such as 413, an unreadable answer, or a per-event answer list of the wrong length) retries forever and blocks the queue; give up on or split the batch after the attempt limit.
- astromech-nhhq: the flight-log request can take 30 s but the app gives up on the core after 15 s; align them so EDSM's own message is shown.

## Done when

Tests: a key change after a 203 sends the held events at once; a batch refused past the attempt limit no longer blocks later events; a slow flight log shows EDSM's message or a specific timeout, not the generic one.
