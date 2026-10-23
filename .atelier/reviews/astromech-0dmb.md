---
schema: atelier.review
schema_version: 1
id: astromech-0dmb
title: 'Re-review of 3dc1e453 (git diff 09822557 3dc1e453): Desk::plotted''s auto-resume path now skips past a waypoint reached while the route was parked (Route::arrive on the current system), matching resume(); fakecore shares one skipReached helper for both paths. The new test case (detour, arrive at N1, plot onward to Colonia -> next N2) exercises it. Proof: cargo test -p astromech-core --lib route:: (13 pass), vitest src/modules/route (14 pass), pnpm check (0 errors/warnings). Limits: Linux only, no real game or Tauri run; no fakecore test of the plotted-resume skip itself.'
target: astromech-g0hm
identity: reviewer-agent-2
commit: 3dc1e45364a824bf3f97a12bc28d331ac484fba1
created_at: 2026-10-23T01:22:20.424310117Z
event:
  event: approval
---

Re-review of 3dc1e453 (git diff 09822557 3dc1e453): Desk::plotted's auto-resume path now skips past a waypoint reached while the route was parked (Route::arrive on the current system), matching resume(); fakecore shares one skipReached helper for both paths. The new test case (detour, arrive at N1, plot onward to Colonia -> next N2) exercises it. Proof: cargo test -p astromech-core --lib route:: (13 pass), vitest src/modules/route (14 pass), pnpm check (0 errors/warnings). Limits: Linux only, no real game or Tauri run; no fakecore test of the plotted-resume skip itself.
