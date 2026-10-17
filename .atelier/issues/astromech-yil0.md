---
created_at: "2026-10-17T16:54:23.417308764+00:00"
id: "astromech-yil0"
issue_type: "task"
labels:
- "alpha"
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-yil0
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Voice recognition eval on a synthetic 200-phrase set"
updated_at: "2026-10-17T20:55:41.063591947+00:00"
---

## Goal

Catch recognition regressions without Sumeet's VR clips: render a 200-phrase command set with Kokoro (several speakers, with and without the Cockpit chain and added noise) and run it through `--eval` (Parakeet, then the intent grammar).

## Done when

The phrase set and the render script are in the repo, one command produces the eval report, and the report (score per command family) is attached as evidence; misses below 95% are filed as issues.

## Context

ARCHITECTURE §14 MVP criterion 2 asks for ≥ 95% on a 200-phrase test set, which does not exist yet. The phrases come from the astromech-intent grammar and aliases, covering every T0/T1 command and every voice lookup. This is a stand-in, not a replacement, for astromech-cgrw (real clips in VR).
