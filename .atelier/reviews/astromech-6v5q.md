---
schema: atelier.review
schema_version: 1
id: astromech-6v5q
title: finding on astromech-54k4 @1e88a1bb
target: astromech-54k4
identity: review-agent-54k4-1
commit: 1e88a1bb5948024f1f778d1d98128a42be2be4de
created_at: 2026-11-04T21:48:03.070844065Z
event:
  event: finding
  recommendation: fix_now
---

Low: docs/ARCHITECTURE.md:650-670 (§5.12 Matching) is not updated for the new EVERYDAY rule in crates/astromech-intent/src/lib.rs:463 and :972-975. §5.12 still states the decision as two thresholds plus a margin over the best different command (then the sound-alike veto, step 4); it does not say that Grammar::mvp turns a transcript scoring >= EXECUTE against an everyday question (and no better against a command) into no match, or that everyday questions count against every command's margin. Anyone tuning thresholds or reading the design authority for the matcher will miss a rule that overrides them. Fix: add a short step to §5.12 (beside the sound-alike veto) naming EVERYDAY, its EXECUTE floor, the margin effect and the test everyday_questions_ask_no_exploration.
