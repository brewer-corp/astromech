---
schema: atelier.review
schema_version: 1
id: astromech-jhc2
title: 'R1 high lib.rs:696 (cap) with :885-887 (margin): best() measures the margin against the capped score, so a capped right answer drops out as rival and a spelling-only neighbour executes where main asked: ''how much full'' -> status.hull, ''how much huel'' -> status.fuel, ''cmose panel''/''cnose panel'' -> panel.close, ''degloy landing gear''/''dekloy landing gear'' -> landing_gear.down. Fix: keep uncapped scores for the runner-up in the margin check; add tests.'
target: astromech-vkzj
identity: reviewer-opus
commit: c98821be1d0fc30e5c5c161ff916c0fb53330be6
created_at: 2026-10-18T05:58:11.965711834Z
event:
  event: finding
  recommendation: fix_now
---

R1 high lib.rs:696 (cap) with :885-887 (margin): best() measures the margin against the capped score, so a capped right answer drops out as rival and a spelling-only neighbour executes where main asked: 'how much full' -> status.hull, 'how much huel' -> status.fuel, 'cmose panel'/'cnose panel' -> panel.close, 'degloy landing gear'/'dekloy landing gear' -> landing_gear.down. Fix: keep uncapped scores for the runner-up in the margin check; add tests.
