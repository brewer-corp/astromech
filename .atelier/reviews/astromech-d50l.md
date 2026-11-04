---
schema: atelier.review
schema_version: 1
id: astromech-d50l
title: finding on astromech-j75v @9f516d3e
target: astromech-j75v
identity: review-agent-j75v-1
commit: 9f516d3e479fae0aa610989b7b29db820e240589
created_at: 2026-11-04T21:25:05.408551449Z
event:
  event: finding
  recommendation: fix_now
---

minor: crates/astromech-core/src/tester.rs:120 and :257 The voice tester now says 'Searches from Sol.' for a site lookup such as 'nearest crystalline shards near Sol', but its Run live button (r.live = the command only; live() calls runner.run_confirmed, which reaches nearest_site(kind, None) at actions.rs:162) searches from the current system, or says the position is not known. The page claims one origin and runs another. Fix: carry the origin into the live run, or leave the live button off when an origin was named.
