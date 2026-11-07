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
    id: "astromech-6i8s"
    type: "related"
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T05:06:25.683162151+00:00"
status: "done"
title: "Idle memory bound for the one-process app"
updated_at: "2026-11-07T05:06:25.683162151+00:00"
---

## Context

Review finding astromech-cg84. The 50 MB was proposed by the planner, not set by Sumeet, and the strategy keeps success criteria with him. The baseline should be pinned: the last two-process release's core, measured with scripts/webview_cost.py on the same machine (on Sumeet's PC that means measuring the old version there too). Decision astromech-6i8s expects a windowless Tauri process to cost a few tens of MB, unmeasured; the bound also depends on the shell's old caches and database connections dropping when idle. Settle before astromech-y6d6 is planned; it doesn't affect astromech-7rcn.

## Question

How much more idle memory may the one-process app use with its window closed than the core does today? The one-process criterion of mission astromech-kjfs (docs/strategies/one-tested-process.md) proposes at most 50 MB above the core alone, and with the window open the window's cost inside S11's 250 MB.

## Decision

No memory bound, decided by Sumeet on 7 Nov 2026: astromech's memory is small next to Elite itself and the browser every player keeps open beside it (Inara), so it isn't worth a success criterion. The one-process criterion keeps only that the app is one process plus the speech worker and that closing the window leaves no WebView2 process; `scripts/webview_cost.py` still reports memory, without a pass mark. This replaces the line "Idle memory must stay inside S11" in decision astromech-6i8s's consequences.

## Close Reason

Decided by Sumeet on 7 Nov 2026: no memory bound; memory is reported, not bounded. Replaces astromech-6i8s's 'Idle memory must stay inside S11'.
