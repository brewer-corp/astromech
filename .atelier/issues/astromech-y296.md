---
created_at: "2026-10-17T21:41:11.650963891+00:00"
id: "astromech-y296"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-y296
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Voice: 'all stop' scores 81% on the synthetic eval (throttle.zero, T1)"
updated_at: "2026-10-17T23:59:39.786752638+00:00"
---

## Goal

Make "all stop" (throttle.zero, a T1 command) recognised reliably. On the synthetic eval (astromech-yil0, docs/evidence/astromech-yil0/report.md) throttle.zero scores 26/32 (81.2%); its family, flight, is 96.2%.

## Done when

throttle.zero scores at least 95% on `scripts/voice-eval.sh` without lowering another command, and no wrong executes appear.

## Context

Parakeet hears "all stop" as "I'll stop." (af_heart, all four conditions) and "Old stopper." / "Old starper." (bm_george); the grammar gives NoMatch. "throttle to zero" scores 100%. Synthetic voices only: check against real clips (astromech-cgrw) before tuning. Options: a MISHEARD entry or an alias, or drop the phrase from the advertised set.

## Notes

- 2026-10-17 [note] Handoff: fix is grammar-level, Ask not Execute. 'I'll' (joined to 'iwill') and 'old' are MISHEARD for 'all', capped at 0.80. voice-eval: throttle.zero 26/32 -> 32/32, wrong execute 0, rejects 64/64, no other command changed. STT biasing re-measured and rejected (docs/evidence/astromech-y296/). Round-1 review findings recorded; R1 and R3 fixed. Approval outstanding (not recorded by the author by instruction).
