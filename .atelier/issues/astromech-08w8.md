---
created_at: "2026-10-17T16:54:23.691494692+00:00"
id: "astromech-08w8"
issue_type: "task"
labels:
- "alpha"
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-08w8
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T00:05:02.268948883+00:00"
status: "done"
title: "MVP criteria harness: mock-game trials, soak, kill switch, action latency"
updated_at: "2026-10-18T00:05:02.268948883+00:00"
---

## Goal

Measure the game-free MVP criteria for astromech-05k2 on the Windows runner against `mockgame`: 200 trials per T1 tool at ≥ 99% verified success, a 4 h soak replay with zero unintended keypresses (`keys.log`), kill switch releasing all keys within 50 ms, and action start ≤ 400 ms p95 after push-to-talk release.

## Done when

One command runs all four and writes a report against each threshold; the report from a Windows run is attached to astromech-05k2 as evidence.

## Context

`crates/astromech-core/examples/mockgame.rs` toggles Status.json flags on keypress with a configurable delay and dropped presses, and logs every key. Device classes beyond keyboard (mouse 4/5, HOTAS, gamepad) need real hardware and stay with the human check.

## Notes

- 2026-10-17 [note] Handoff (Claude): PR #119. scripts/mvp_criteria.py plus the MVP criteria workflow (5 min soak per PR touching the core; manual 4 h run). Full run 39062129341 passed all four (estimate at +3.6 ms). Evidence on astromech-05k2. Findings filed: astromech-e0zz (PTT release acted on ~210 ms late, no margin), astromech-lnzj (GuiFocus left/right values). Deferred review items: astromech-ty92, astromech-3vjv. Independent review done, FIX NOW items fixed and re-reviewed; awaiting Sumeet's approval. Not covered: mouse 4/5, HOTAS, gamepad, speech, the real game.

## Close Reason

MVP criteria harness on the Windows runner against the mock game: full run 200/200 per T1 tool, 0 unintended keypresses in 240 min, kill switch max 1.2 ms, action start ~396 ms p95 (estimate; astromech-e0zz). Short run per PR; full run on dispatch. Reviewed; a tool-name drift check added.
