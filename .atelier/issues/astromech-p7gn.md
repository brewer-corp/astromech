---
created_at: "2026-10-17T02:45:29.716751252+00:00"
id: "astromech-p7gn"
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
title: "Move the visit-history lookup off the pipeline thread"
updated_at: "2026-10-17T02:45:29.716751252+00:00"
---

## Goal

Keep the arrival visit-history lookup off the event pipeline thread.

## Done when

- `HistoryVisits::before` (crates/astromech-core/src/visits.rs) no longer runs synchronously on the pipeline thread.
- It is precomputed or moved to a worker, so a slow open or a busy wait of up to 50 ms can't delay other callouts.
- This came from the review of astromech-2myu, finding R1 (low).
