---
schema: atelier.review
schema_version: 2
id: astromech-uerh
title: finding on astromech-vucf @2348c8e9
target: astromech-vucf
identity: claude-reviewer-i1ev
commit: 2348c8e90388cdd90cf1e69e800cfa34ccafd91d
created_at: 2026-11-07T16:22:23.909862164Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/pipeline.rs:235 (may_change) and its uses at pipeline.rs:1155, history.rs:110, health.rs:209. Nothing guards the filter's wiring. Reviewer mutation: with may_change returning true, all 8 world_steps tests still pass. With the filter off AND Pipeline::drop's wake removed, the_game_exiting_stops_the_session_with_no_more_time passes 5/5; with the filter on, that mutant fails at the 30 s guard. So the stop test's power, and every advance's quiescence on Linux, depend on a filter that only a predicate unit test (a_read_is_not_a_change) covers. If the filter is dropped or a fourth watcher skips it, the self-wake loop comes back and no test notices. Fix: a Linux test that reads a watched file through a real watcher and shows no event reaches the clock channel (for example a read, then a sentinel write, and only the sentinel's event arrives), or route every watcher through one constructor that applies the filter. Defer to astromech-vucf.
