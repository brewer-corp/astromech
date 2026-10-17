---
created_at: "2026-10-17T06:16:20.758928051+00:00"
id: "astromech-egyl"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-egyl
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T07:02:00.662773311+00:00"
status: "done"
title: "Trade sell and buy lookups: say when the answer was cut at 1000 markets"
updated_at: "2026-10-17T07:02:00.662773311+00:00"
---

## Goal

The truncation flag (Ardent's 1000-row cut) is shown only by the cargo plan's re-ask and colonisation; the sell and buy lookups never surface it, so a cut answer looks complete (finding astromech-txcr on astromech-243w). Show it the way app/src/lib/colonisation.ts does ("narrow the radius").

## Done when

A page test: a truncated sell answer shows the narrow-the-radius hint; a full one doesn't.

## Close Reason

Nearby Sell and Buy lookups show the narrow-the-radius hint when Ardent cut the answer; page test covers cut and full
