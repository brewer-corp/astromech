---
schema: atelier.review
schema_version: 1
id: astromech-4ttq
title: 'R2 gear_down jumping_into_a_waypoint_copies_and_announces_the_next: the catch-up-is-silent assertion runs before any route exists, so replacing if !first with if true survives every test.'
target: astromech-1135
identity: claude-reviewer-1 (relayed by the coordinator)
commit: 4002f5957f28777eb7c6d165d73ba39800acd777
created_at: 2026-10-17T17:37:25.123408135Z
event:
  event: finding
  recommendation: fix_now
---

R2 gear_down jumping_into_a_waypoint_copies_and_announces_the_next: the catch-up-is-silent assertion runs before any route exists, so replacing if !first with if true survives every test.
