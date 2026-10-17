---
created_at: "2026-10-17T17:13:42.109076861+00:00"
id: "astromech-xca4"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-xca4
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Sustained EPS and clamp: charged weapons and engineered reloads"
updated_at: "2026-10-17T22:38:14.316460715+00:00"
---

## Goal

Sustained EPS and time to drain use the same reload cycle as sustained DPS for charged weapons, and sustained DPS never exceeds burst for engineered builds.

## Done when

- Sustained EPS and time to drain use a cycle consistent with sustained DPS for charged weapons (found in review of astromech-tgar; today sustained EPS counts clip/rof plus reload, the DPS cycle counts charge but leaves out the last shot's interval), or the difference is documented as deliberate.
- An engineered build with the reload below the fire interval no longer shows sustained above burst (clamp the factor at 1, or count the last shot's interval), with a test on an engineered rail gun.
