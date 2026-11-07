---
schema: atelier.review
schema_version: 2
id: astromech-gcve
title: approval on astromech-7149 @3025130d
target: astromech-7149
identity: claude-reviewer-batch1
commit: 3025130d74f55541998dae84b85a0bc57b902253
created_at: 2026-11-07T13:12:35.396456561Z
event:
  event: approval
---

Review of 3025130d (test-only, site_lookup.rs). Cause right: the old test counted the test thread's opens after the lookup spawned its retry; when the retry's open finished first, the test's get found the map open and its count was 1, not 2. GalaxyHolder::get serialises opens under its opening lock and re-peeks, so this is test timing, not a product bug. The new test still proves: NO_STAR_MAP spoken alone (spoken_alone), the retry started on a thread other than the speech thread (second open report != test thread), the retry succeeded (the test's get then opens nothing: try_recv Empty), and the next lookup answers ANSWER. Deterministic: waits on channel events in order, no sleeps, polling or retries; a regression that drops the retry would hang recv rather than fail, caught by CI's job timeout. Fresh: site_lookup 50 runs with --test-threads=8, 50/50 passed (8 tests each).
