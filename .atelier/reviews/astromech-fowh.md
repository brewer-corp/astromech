---
schema: atelier.review
schema_version: 2
id: astromech-fowh
title: approval on astromech-vucf @2348c8e9
target: astromech-vucf
identity: claude-reviewer-i1ev
commit: 2348c8e90388cdd90cf1e69e800cfa34ccafd91d
created_at: 2026-11-07T16:22:25.285581294Z
event:
  event: approval
---

Review of i1ev (b89bc62c, 5197dd65, b022c12a, 2348c8e9). Read settle.rs, steps.rs, script.rs, the world.rs, pipeline, history, health, session, app and clock diffs. Settling is event-driven: FileGens bumps under the lock the pipeline's seen() takes, seen() runs before reads and handled() after, a condvar plus a real-time guard only. Sessions counts both ends of activate. The gate uses wait_others_parked on the manual clock's settled condvar, notified on every park and unregister. A game step polls at most MISSES_TO_IDLE times and checks core.game after each quiescent advance. The installed app gets settle None and main.rs passes None, so each hook is an Option check. The watched() refactor and the activate/start_session split are equivalent. may_change is checked against notify 8.2.0 (Cargo.lock): inotify maps OPEN and CLOSE_NOWRITE to the dropped Access kinds, and real writes still arrive as Modify, Create, Name or Close(Write). The Windows backend emits no Access events, so in-play behaviour there is unchanged. 8t2s is fixed: Following::stop is final, and only world mode calls it. worlds/jump holds hand-built wiki-visible data only. Done when 5's rewording is honest apart from finding 2. Fresh runs: core lib 245 passed; world_steps and world_mode pass; world_steps looped 50/50, then 4 concurrent loops x15 60/60, no temp worlds left. Mutations in a scratch export: no stop-wake fails the stop test at the guard; no file-generation wait fails the partial test 5/5; filter off is undetected (finding 1). cargo test --workspace: 1351 passed, 0 failed, 136 binaries. clippy -D warnings, fmt, check-deps and check-guide clean. Not run: cargo deny, Windows.
