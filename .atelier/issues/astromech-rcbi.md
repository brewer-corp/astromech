---
created_at: "2026-10-18T00:10:36.877950116+00:00"
id: "astromech-rcbi"
issue_type: "task"
labels:
- "voice"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Voice: narrow the 'all stop' mishearing and pin near-misses below execute"
updated_at: "2026-10-18T00:10:36.877950116+00:00"
---

## Goal

Tighten the "all stop" mishearing so it asks only when the second word is close to "stop", and pin the plain fuzzy near-misses below execute.

## Done when

"ill stop" and "Al stop" (0.833 by fuzzy match alone, 0.017 under EXECUTE) have a test that they never execute; "I'll jump", "I'll paste", "I'll start" no longer ask "Did you mean all stop?" if real clips (astromech-cgrw) show those wrong asks happen; the voice eval keeps throttle.zero at or above 95% with 0 wrong executes.

## Context

Review findings R5 and R6 of astromech-y296 (round 2). "I'll" and "old" are MISHEARD for "all" (capped 0.80), so "I'll X" half-matches "all stop" and asks about the throttle. Nothing is pressed without a yes. Check against real clips first.
