---
schema: atelier.review
schema_version: 1
id: astromech-jv5m
title: finding on astromech-sllo @1fd71219
target: astromech-sllo
identity: reviewer-fable-sllo
commit: 1fd712195d1b0ebd90f65ccdd3d7f695e3f16e83
created_at: 2026-11-05T18:50:34.134803598Z
event:
  event: finding
  recommendation: fix_now
---

low: DESIGN.md:256 the header comment still says 'Still to be brought in line: astromech-sllo (two tab rows)'; after this branch every page shows at most two tab rows (tabrows.test.ts), so the binding doc would be stale the moment it merges and an agent reading it would think the rule is not yet applied. Fix now: drop the clause or say the pages moved to two tab rows with astromech-sllo, in this branch.
