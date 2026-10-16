---
schema: atelier.review
schema_version: 1
id: astromech-y8ys
title: 'R1: events.rs:140 reads ApproachBody''s body from BodyName; real events carry Body, so codexentry never gets BodyID after a descent; fixture masks it.'
target: astromech-tyjz
identity: review-agent (tyjz round 4, w20r/5el6)
commit: 68db54384713eb9c404d0ce2569f7891cfb9feb9
created_at: 2026-10-16T19:39:38.040712625Z
event:
  event: finding
  recommendation: fix_now
---

R1: events.rs:140 reads ApproachBody's body from BodyName; real events carry Body, so codexentry never gets BodyID after a descent; fixture masks it.
