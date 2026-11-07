---
created_at: "2026-11-07T04:57:04.937890245+00:00"
id: "astromech-01ft"
issue_type: "decision"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Idle memory bound for the one-process app"
updated_at: "2026-11-07T04:57:04.937890245+00:00"
---

## Context

Review finding astromech-cg84. The 50 MB was proposed by the planner, not set by Sumeet, and the strategy keeps success criteria with him. The baseline should be pinned: the last two-process release's core, measured with scripts/webview_cost.py on the same machine (on Sumeet's PC that means measuring the old version there too). Decision astromech-6i8s expects a windowless Tauri process to cost a few tens of MB, unmeasured; the bound also depends on the shell's old caches and database connections dropping when idle. Settle before astromech-y6d6 is planned; it doesn't affect astromech-7rcn.

## Question

How much more idle memory may the one-process app use with its window closed than the core does today? The one-process criterion of mission astromech-kjfs (docs/strategies/one-tested-process.md) proposes at most 50 MB above the core alone, and with the window open the window's cost inside S11's 250 MB.
