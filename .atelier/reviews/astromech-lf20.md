---
schema: atelier.review
schema_version: 2
id: astromech-lf20
title: approval on astromech-vucf @d0ddf33e
target: astromech-vucf
identity: claude-reviewer-u8ge
commit: d0ddf33e7740c22577dfc469dcb5a9adf44e5846
created_at: 2026-11-07T15:06:19.500164884Z
event:
  event: approval
---

Re-review of u8ge's fixes (e6b1644d, records d0ddf33e). r2ar fixed: history settle is a recv_timeout loop on the follower's clock channel to the same deadline; under SystemClock it ends at the same time except that a stop ends it at once (no import is skipped: before, the post-settle loop top returned on stop too); disconnect still sits out the rest; teardown exception 3 recorded in Done when 1/3 and the Report, false 'nothing hangs' corrected; re_pointing_during_the_settle_lets_the_clock_settle passes. p78l fixed: Manual::begin checks an absolute deadline and inserts under one lock; the hook test forces the gap. wait_timeout_from/sleep_from checked against main: SystemClock hub wait_flag/wait_gui are cv.wait_timeout(g, (end-now).min(25ms)) with the same single now read and kill-switch slices; policy wait_foreground reads now once, compares, thread::sleep(50ms), identical to main. Proof at d0ddf33e: cargo test -p astromech-proto -p astromech-policy -p astromech-core 338 passed plus 1 known world_mode quit flake (support/mod.rs:109); 40-iteration loop of proto, core hub/history/pipeline and policy tests 0/120 failures; clippy -D warnings on the three crates and fmt clean. world_mode 60 runs: 1 known killed-parent 'Directory not empty' flake; neither flake path is touched by this diff (open with the orchestrator, d2nq teardown). Windows CI still outstanding (Done when 9 unticked).
