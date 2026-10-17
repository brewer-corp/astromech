---
schema: atelier.review
schema_version: 1
id: astromech-aqrl
title: 'R2 [low] crates/astromech-core/tests/mining_status.rs and crates/astromech-gamestate/tests/mining.rs: the fixture''s last line is a MiningRefined, so nothing tests the new MiningRun.last advancing on non-mining lines after the last refine, which is the documented ''measured to the latest journal line'' behaviour. Fix: add a later non-mining line and assert per_hour drops. Recommendation: DEFER (behaviour checked by inspection).'
target: astromech-p9r4
identity: review-agent (opus, independent)
commit: 0815c73f5f7e42021269493f59bb4d5720705ef3
created_at: 2026-10-17T04:55:16.253765865Z
event:
  event: finding
  recommendation: defer
---

R2 [low] crates/astromech-core/tests/mining_status.rs and crates/astromech-gamestate/tests/mining.rs: the fixture's last line is a MiningRefined, so nothing tests the new MiningRun.last advancing on non-mining lines after the last refine, which is the documented 'measured to the latest journal line' behaviour. Fix: add a later non-mining line and assert per_hour drops. Recommendation: DEFER (behaviour checked by inspection).
