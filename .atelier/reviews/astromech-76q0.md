---
schema: atelier.review
schema_version: 1
id: astromech-76q0
title: '[low] crates/astromech-core/src/pipeline.rs:292-301 with crates/astromech-announce/src/route_follow.rs:113-121: after an older journal becomes newest (backup tool touch), the switch back to the current journal re-opens it from byte 0 as a replay (open(n, !live), live = !replay), so the commander''s next real jump, which is the line that triggers the switch back, reaches the follower as replayed(): the route does not move and the clipboard keeps the waypoint the commander is now in for one leg (it heals at the next waypoint, since Route::arrive searches forward). The new test asserts this gap itself (gear_down.rs:640-645, the ''Sol'' jump is silent). Pre-existing pipeline semantics shared with EDDN and all callouts, not introduced here. Suggest a follow-up: when re-opening a journal already read this run, replay up to its previously read offset and treat later lines as live.'
target: astromech-1135
identity: claude-reviewer-2
commit: 642cafe0a3bc7073503113bcc662608b12300bec
created_at: 2026-10-17T17:39:52.733075360Z
event:
  event: finding
  recommendation: defer
---

[low] crates/astromech-core/src/pipeline.rs:292-301 with crates/astromech-announce/src/route_follow.rs:113-121: after an older journal becomes newest (backup tool touch), the switch back to the current journal re-opens it from byte 0 as a replay (open(n, !live), live = !replay), so the commander's next real jump, which is the line that triggers the switch back, reaches the follower as replayed(): the route does not move and the clipboard keeps the waypoint the commander is now in for one leg (it heals at the next waypoint, since Route::arrive searches forward). The new test asserts this gap itself (gear_down.rs:640-645, the 'Sol' jump is silent). Pre-existing pipeline semantics shared with EDDN and all callouts, not introduced here. Suggest a follow-up: when re-opening a journal already read this run, replay up to its previously read offset and treat later lines as live.
