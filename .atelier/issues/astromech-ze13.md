---
created_at: "2026-10-15T20:36:13.650802920+00:00"
id: "astromech-ze13"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-ze13
priority: "P1"
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
status: "in_progress"
title: "Material cap alerts and trade-down suggestions"
updated_at: "2026-10-17T02:48:20.126034793+00:00"
---

## Goal

Alert when a material hits its cap ("Arsenic at 300/300") and suggest what to trade down at a trader.

## Done when

Replaying a fixture that fills a material raises one alert; docking at a trader suggests trades that free that slot.

## Context

A reaction rule over the materials ledger.
