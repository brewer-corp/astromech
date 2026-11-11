---
schema: atelier.item
schema_version: 3
id: astromech-p7gn
title: Move the visit-history lookup off the pipeline thread
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T02:45:29.716751252Z
---

## Goal

Keep the arrival visit-history lookup off the event pipeline thread.

## Done when

- [ ] - `HistoryVisits::before` (crates/astromech-core/src/visits.rs) no longer runs synchronously on the pipeline thread. - It is precomputed or moved to a worker, so a slow open or a busy wait of up to 50 ms can't delay other callouts. - This came from the review of astromech-2myu, finding R1 (low).
