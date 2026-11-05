---
schema: atelier.review
schema_version: 1
id: astromech-aca0
title: finding on astromech-7ew2 @690e0fd9
target: astromech-7ew2
identity: review-agent-7ew2-1
commit: 690e0fd9cc9067d68f289e5dfadb78dba476dac5
created_at: 2026-11-04T23:17:38.786435195Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-eddn/src/edsm/mod.rs:513-524 (tests.rs:752) The invariant that a failed refused batch or half counts no try against its events (astromech-sklz: counted per event only once down to one) is not pinned. Mutation check: adding refuse_at(refusals+1) for every row in that branch leaves all 40 edsm tests green, including the new poisoned-drain test, because the innocent neighbours stay under MAX_ATTEMPTS in that scenario. A regression would quietly use up refusals of good events near a poison and could give them up (data loss). Suggested fix: in the poisoned-drain test (or a small pass-level test with refused_once and one poison), assert the stored events' neighbours have the same refusals before and after the halving, or that only the poison's refusals grew.
