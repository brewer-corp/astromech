---
schema: atelier.review
schema_version: 1
id: astromech-k8zj
title: 'Low, crates/astromech-galaxy/tests/plot.rs:106: the exact-plotter path is tested only with the Asp''s 4, the value the old code hard-coded, so a regression that set 4 in plot.rs:193 instead of f.supercharge_multiplier would still pass (the neutron test does use 6, and astromech-data proves the form carries 6). Optional: set ship.fsd.supercharge_multiplier = 6 in an_exact_route_has_every_jump_and_its_refuel_stops as the neutron test does.'
target: astromech-vlsz
identity: review-agent-vlsz-1
commit: f51ba497e7c94995eb71af38514d7422dfa71290
created_at: 2026-10-22T20:50:56.377800554Z
event:
  event: finding
  recommendation: no_action
---

Low, crates/astromech-galaxy/tests/plot.rs:106: the exact-plotter path is tested only with the Asp's 4, the value the old code hard-coded, so a regression that set 4 in plot.rs:193 instead of f.supercharge_multiplier would still pass (the neutron test does use 6, and astromech-data proves the form carries 6). Optional: set ship.fsd.supercharge_multiplier = 6 in an_exact_route_has_every_jump_and_its_refuel_stops as the neutron test does.
