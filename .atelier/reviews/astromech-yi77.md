---
schema: atelier.review
schema_version: 1
id: astromech-yi77
title: finding on astromech-k1fw @57bbf76d
target: astromech-k1fw
identity: review-agent (sonnet, independent)
commit: 57bbf76dedd50a047b2c77b37f5b0d3fdcac9595
created_at: 2026-11-05T01:03:41.973632953Z
event:
  event: finding
  recommendation: fix_now
---

high: crates/astromech-announce/src/route_follow.rs:88 (with astromech-announce/src/lib.rs:434-441): a clear is dropped for good when the journal line is read before the empty NavRoute.json. The guard ignores NavRouteClear while state.route_hops has hops, but the empty file's later read only calls apply_route; Engine::route returns early on empty hops, so nothing reaches the rule and the follower never gets cleared(). Journal-only wakes (on_journal_activity, on_idle read_tail) do not re-read NavRoute.json, and the game's order of the journal line versus the file rewrite is undocumented. Scenario: plot route A, clear in game, journal line read first, file still has A: clear ignored, then the empty file arrives and is swallowed, so the Route page keeps showing the cleared route. The new test only covers file-empty-first. Fix: deliver an empty route that follows a non-empty last_route to the follower as cleared() (a new Input or clear path), or remember an ignored clear and apply it when the file goes empty; add a test with the journal line first and the empty file after.
