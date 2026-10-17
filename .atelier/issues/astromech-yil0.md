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
closed_at: "2026-10-17T22:08:35.463096148+00:00"
status: "done"
title: "Voice recognition eval on a synthetic 200-phrase set"
updated_at: "2026-10-17T22:08:35.463096148+00:00"
---

## Goal

Catch recognition regressions without Sumeet's VR clips: render a 200-phrase command set with Kokoro (several speakers, with and without the Cockpit chain and added noise) and run it through `--eval` (Parakeet, then the intent grammar).

## Done when

The phrase set and the render script are in the repo, one command produces the eval report, and the report (score per command family) is attached as evidence; misses below 95% are filed as issues.

## Context

ARCHITECTURE §14 MVP criterion 2 asks for ≥ 95% on a 200-phrase test set, which does not exist yet. The phrases come from the astromech-intent grammar and aliases, covering every T0/T1 command and every voice lookup. This is a stand-in, not a replacement, for astromech-cgrw (real clips in VR).

## Notes

- 2026-10-17 [note] Handoff: phrase set crates/astromech-core/tests/voice-eval-phrases.tsv (200, every MVP command); scripts/voice-eval.sh renders (4 Kokoro voices x clean/cockpit/noise/cockpit+noise), transcribes with Parakeet and scores via eval.rs; first report docs/evidence/astromech-yil0/report.md: 3105/3136 (99%), 0 wrong executes, every family >= 95%. Commands below 95% filed: y296 (throttle.zero 81%, alpha), bztt (confirm 94%, alpha), 8fey (paste 94%, alpha), xi2i (status/hyperspace). Reviewed twice, all findings fixed; no approval recorded. Run takes ~40 min on this host.

## Close Reason

Synthetic 200-phrase eval built and run: 3105/3136 correct (99%), 0 wrong executes, every family >= 95%; commands under 95% filed (astromech-y296, -bztt, -8fey alpha; -xi2i). Reviewed twice.
