---
schema: atelier.review
schema_version: 1
id: astromech-69wx
title: 'R2 [low] crates/astromech-eddn/src/edsm/mod.rs:374-385 with app.rs apply_edsm_switch: after a 201-203 answer (wrong name or key), every queued event is pushed back by backoff, up to MAX_BACKOFF_SECS (an hour). Correcting the name or key in Settings does not reset next_try or wake the worker, so the upload resumes up to an hour later and the page shows the stale 203 as the last error. Fix: when the credentials change, make held rows due now and wake the worker.'
target: astromech-ah0i
identity: review-agent (opus, independent)
commit: aa58b47beb073e878c106e16878529bca17d43f1
created_at: 2026-10-17T05:04:00.326318183Z
event:
  event: finding
  recommendation: defer
---

R2 [low] crates/astromech-eddn/src/edsm/mod.rs:374-385 with app.rs apply_edsm_switch: after a 201-203 answer (wrong name or key), every queued event is pushed back by backoff, up to MAX_BACKOFF_SECS (an hour). Correcting the name or key in Settings does not reset next_try or wake the worker, so the upload resumes up to an hour later and the page shows the stale 203 as the last error. Fix: when the credentials change, make held rows due now and wake the worker.
