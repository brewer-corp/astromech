---
schema: atelier.review
schema_version: 1
id: astromech-ynwo
title: 'EDDN merge 9c7644e (plus conflict-free ba665fd) approved. Shared live rule: callouts fire only when caught_up && Tailer::live, EDDN uses Tailer::live, Status.json callouts when caught_up. So the startup replay and mid-session re-reads of older journals are silent for both, and a later journal is live from its first line. Order per line: hub.apply_journal, then the callout engine (reading the hub), then the EDDN feed. Companions and NavRoute still come after the lines. Reactions are gathered inside the stats lock and delivered after it is dropped (on_status delivers with no stats lock). EDDN core code is unchanged from main apart from the start_with_eddn parameter. set_eddn still goes through edit_config, and eddn_log is kept. Engine::go_live became set_live(bool) with every caller updated. No test from either parent is lost, and the new sub-assertion fails under the old engine-only boundary (4 replayed scans would fire). Checks run, all green: fmt, clippy -D warnings, cargo test --workspace, check-deps, src-tauri metadata --locked, pnpm check/test (82). One low deferred finding (astromech-hc73), pre-existing in both parents. Correction to it: DOCKING_ENDS includes Shutdown, so the scenario needs a journal that ends without one, as after a crash.'
target: astromech-cmhv
identity: review-agent (cmhv merge review)
commit: ba665fd1a287b479b2fc19ae66b2d3f69cffdecb
created_at: 2026-10-16T22:35:10.189441370Z
event:
  event: approval
---

EDDN merge 9c7644e (plus conflict-free ba665fd) approved. Shared live rule: callouts fire only when caught_up && Tailer::live, EDDN uses Tailer::live, Status.json callouts when caught_up. So the startup replay and mid-session re-reads of older journals are silent for both, and a later journal is live from its first line. Order per line: hub.apply_journal, then the callout engine (reading the hub), then the EDDN feed. Companions and NavRoute still come after the lines. Reactions are gathered inside the stats lock and delivered after it is dropped (on_status delivers with no stats lock). EDDN core code is unchanged from main apart from the start_with_eddn parameter. set_eddn still goes through edit_config, and eddn_log is kept. Engine::go_live became set_live(bool) with every caller updated. No test from either parent is lost, and the new sub-assertion fails under the old engine-only boundary (4 replayed scans would fire). Checks run, all green: fmt, clippy -D warnings, cargo test --workspace, check-deps, src-tauri metadata --locked, pnpm check/test (82). One low deferred finding (astromech-hc73), pre-existing in both parents. Correction to it: DOCKING_ENDS includes Shutdown, so the scenario needs a journal that ends without one, as after a crash.
