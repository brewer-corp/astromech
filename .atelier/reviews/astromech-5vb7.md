---
schema: atelier.review
schema_version: 1
id: astromech-5vb7
title: 'The greedy payment can overshoot where an exact payment exists: owed 15 with spare 1x11 and 3x5 gives 11+5=16 rather than 3x5=15 (barter.rs:88-92), and the doc comment''s ''so the fewest items change hands'' is not guaranteed. Losses are at most one item''s value per swap and the page shows the overshoot, so no change needed now.'
target: astromech-ha4m
identity: review-agent (opus, independent)
commit: a0d17a74f115194c3f7ffd52d8703a4beaf69daa
created_at: 2026-10-17T05:27:06.775667740Z
event:
  event: finding
  recommendation: no_action
---

The greedy payment can overshoot where an exact payment exists: owed 15 with spare 1x11 and 3x5 gives 11+5=16 rather than 3x5=15 (barter.rs:88-92), and the doc comment's 'so the fewest items change hands' is not guaranteed. Losses are at most one item's value per swap and the page shows the overshoot, so no change needed now.
