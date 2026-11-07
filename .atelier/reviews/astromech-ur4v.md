---
schema: atelier.review
schema_version: 2
id: astromech-ur4v
title: finding on astromech-vucf @86b4d3e8
target: astromech-vucf
identity: claude-reviewer-xk0n
commit: 86b4d3e81bb36e6f278617b25d59ae0d32c6e4b9
created_at: 2026-11-07T17:13:03.427001863Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/world.rs:504-505 (NO_WAIT) with crates/astromech-data/src/spansh.rs:1050-1061 (spansh_wait). In world mode Spansh jobs poll with every=0 on the manual clock, and spansh_wait's 180 s timeout reads that clock, which a lookup never moves. A world whose recording answers a job's results as still queued (an Always exchange, or an in_turn that falls through to one) makes the lookup spin at full CPU forever: the control call never returns and nothing fails it. Found by reading; not run. Fix: bound the polls in world mode (a poll count, or a timeout on real time that only fails the call, like world_wait's guard). Defer to astromech-gllh, the first child whose worlds plot routes.
