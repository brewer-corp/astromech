---
schema: atelier.review
schema_version: 1
id: astromech-ycv5
title: finding on astromech-7ew2 @690e0fd9
target: astromech-7ew2
identity: review-agent-7ew2-1
commit: 690e0fd9cc9067d68f289e5dfadb78dba476dac5
created_at: 2026-11-04T23:17:39.252767171Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-eddn/src/edsm/tests.rs:752-760 The poisoned-drain test pins one position (index 100); the implementer's checks at 0, 1, 50, 199 were ad hoc and not committed. Reviewer probe (scratch copy, not committed) at 0,1,2,48,49,50,51,99,100,150,198,199: 26-34 requests, 200 stored once each, poison given up; two poisons [10,150] take 52 and five spread poisons 97, still far below 201 but above the test's 5x bound. Suggested fix: loop drain_after_an_outage over a few positions such as 0, 49, 50 and 200 (batch edges and both ends), about 0.1 s each, so a boundary bug in the halving or doubling cannot pass on index 100 alone.
