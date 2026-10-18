---
created_at: "2026-10-17T21:41:11.818166743+00:00"
id: "astromech-bztt"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-bztt
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Voice: 'Yes, sir.' is rejected (confirm scores 94% on the synthetic eval)"
updated_at: "2026-10-18T00:11:44.916356670+00:00"
---

## Goal

Accept a confirmation with a polite extra word. On the synthetic eval (astromech-yil0, docs/evidence/astromech-yil0/report.md) Parakeet transcribes bm_george's "yes" as "Yes, sir." in three of four conditions and the grammar gives NoMatch, so confirm scores 45/48 (93.8%). The confirm family (confirm, cancel) is 96.9%.

## Done when

"Yes, sir." (and similar "yes please") confirms, confirm scores at least 95% on `scripts/voice-eval.sh`, and no wrong executes appear.

## Context

The rendered text was "yes.": the "sir" is Parakeet adding a word on that voice, not a word the speaker said. Either way the grammar rejects a one-word phrase plus one extra word, and a commander may well say "yes sir". Check against real clips (astromech-cgrw) first. Confirm answers the silent-running and did-you-mean prompts, so it is part of ship control.

## Notes

- 2026-10-17 [note] Handoff: 'sir' added to FILLER. voice-eval confirm 45/48 -> 48/48, wrong execute 0, rejects 64/64, no other command changed. Review round 1: no FIX NOW; R3 deferred to astromech-kdca. Approval outstanding (not recorded by the author by instruction).
