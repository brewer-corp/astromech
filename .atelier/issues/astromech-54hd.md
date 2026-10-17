---
created_at: "2026-10-16T18:56:53.832131635+00:00"
id: "astromech-54hd"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-54hd
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-q318"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:33:33.551073116+00:00"
status: "done"
title: "Merged controller page: the other stick and per-action detail"
updated_at: "2026-10-17T19:33:33.551073116+00:00"
---

## Goal

Merging My controller into Activities (astromech-g5rw) dropped things Activities had: with an activity chosen, actions bound on the other stick show neither as on this stick nor as elsewhere; the per-action detail panel (slots, health warnings, Bind another control) is gone; the activity choice is no longer remembered; and the Ship/General preference for shared boxes is gone.

## Done when

With an activity chosen, actions on the other stick are listed; clicking an action opens its detail with Bind another control; the activity choice survives a reload. Component tests for each.

## Context

`app/src/modules/controller/View.svelte`. Found in review of astromech-q318 (round 1, R3). Ask Sumeet whether both sticks side by side are wanted back before v1.0.

## Close Reason

Done-when met: actions on the other stick are listed, clicking an action opens its detail with Bind another control, the activity survives a reload; component tests for each; PR 133; see evidence
