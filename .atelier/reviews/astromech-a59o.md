---
schema: atelier.review
schema_version: 2
id: astromech-a59o
title: review on astromech-bp23 @ad9ad743
target: astromech-bp23
identity: claude-reviewer-batch1
commit: ad9ad743534d4cc42681c9bdcec10cbf34aad7d1
created_at: 2026-11-07T08:05:18.831970127Z
event:
  event: review
---

Review of k6da (6f1b48c8), 96ci (6547b879), ravn (d82d387d) and their close records against each Done when, bp23, unh9/1zhg/6i8s and the one-tested-process Authority section. Not approved: two fix_now findings (u6ak spoken-behaviour change in voice lookups needs Sumeet or a cheap restore; p3tv four Condvar waits still unwrap a poisoned lock). Read state.rs (ForDir/GalaxyHolder/Kept stamps, release_idle), Galaxy::unload_map (same as invalidate; in-flight users keep their Arc), the reaper wiring, actions.rs/assistant.rs look_up, lib.rs dispatching/Recover, crash.rs hook, every non-test .recover() site in astromech-core src, session.rs/biology.rs, paths.rs deletion. Recovery is safe at each site: critical sections are single assignments, pushes or derived state (GameState, route desk, caches) whose partial update is no worse than the old cascade of panics. Idle release vs in-flight: callers keep their Arc; the next get reloads; no correctness issue, at worst a brief second neutron load. Fresh: cargo test --workspace 1289 passed 0 failed 2 ignored; clippy -D warnings on api/core/galaxy/store all-targets clean; fmt clean; check-deps OK (32 crates); cargo deny ok. pnpm not rerun: the three commits touch no app/ files. Mutation checks not rerun.
