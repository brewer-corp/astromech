---
created_at: "2026-10-17T21:41:11.650963891+00:00"
id: "astromech-y296"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice: 'all stop' scores 81% on the synthetic eval (throttle.zero, T1)"
updated_at: "2026-10-17T21:41:11.650963891+00:00"
---

## Goal

Make "all stop" (throttle.zero, a T1 command) recognised reliably. On the synthetic eval (astromech-yil0, docs/evidence/astromech-yil0/report.md) throttle.zero scores 26/32 (81.2%); its family, flight, is 96.2%.

## Done when

throttle.zero scores at least 95% on `scripts/voice-eval.sh` without lowering another command, and no wrong executes appear.

## Context

Parakeet hears "all stop" as "I'll stop." (af_heart, all four conditions) and "Old stopper." / "Old starper." (bm_george); the grammar gives NoMatch. "throttle to zero" scores 100%. Synthetic voices only: check against real clips (astromech-cgrw) before tuning. Options: a MISHEARD entry or an alias, or drop the phrase from the advertised set.
