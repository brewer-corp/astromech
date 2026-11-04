---
schema: atelier.review
schema_version: 1
id: astromech-ys8x
title: finding on astromech-az6g @32ef569f
target: astromech-az6g
identity: review-agent-az6g-1
commit: 32ef569f95400a7642fd26e6baa8d5c17e458182
created_at: 2026-11-04T21:29:17.994669736Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-eddn/src/edsm/tests.rs:725 Only the happy path of rate_limit is tested (Remaining 0, Reset 600). Nothing tests a missing Reset (60 s), garbage or negative Reset (falls back to 60 s), a huge Reset or an epoch timestamp (capped at 3600 s), or a Remaining that isn't a plain integer (ignored). By inspection, mod.rs:433-447 handles all of these safely. Suggested: a small table test over rate_limit's outcomes so a later edit can't turn a bad header into a stall longer than an hour.
