---
created_at: "2026-10-16T20:11:50.449433736+00:00"
id: "astromech-p9r4"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-p9r4
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-cmhv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show the mining run: tons refined and tons per hour"
updated_at: "2026-10-17T03:23:09.058665176+00:00"
---

## Goal

While mining, the app shows the current run: tons refined (by ore) and tons per hour, so Sumeet can judge a spot without a separate tool.

## Done when

The core keeps a `MiningRun` (`astromech_gamestate::mining`) fed from the journal pipeline and includes it in its status reply; a page in the window shows refined tons by ore and tons per hour; a fixture test covers the status shape and the page renders it (screenshot).

## Context

astromech-omi1 built the computation (`MiningRun`: a run starts at the first prospector launch, prospect or refine after docking or loading; one ton per `MiningRefined`), tested against a mining fixture, but nothing shows it yet. There is no session view in the window today.
