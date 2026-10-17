---
schema: atelier.review
schema_version: 1
id: astromech-fa10
title: R2 [LATER] tests/known_bodies.rs a_live_jump_gets_the_bodies_known_line allows ~20 ticks x 20 ms (~400 ms) for the lookup thread; could miss on a loaded CI box. Widen to ~100 ticks.
target: astromech-8sx3
identity: review-agent (sonnet, independent)
commit: 3e737354fd51ffb9b76004f3ef86672a3f581876
created_at: 2026-10-17T19:23:49.337823884Z
event:
  event: finding
  recommendation: defer
---

R2 [LATER] tests/known_bodies.rs a_live_jump_gets_the_bodies_known_line allows ~20 ticks x 20 ms (~400 ms) for the lookup thread; could miss on a loaded CI box. Widen to ~100 ticks.
