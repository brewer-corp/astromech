---
created_at: "2026-10-15T20:36:13.656019516+00:00"
id: "astromech-w62j"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-w62j
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
closed_at: "2026-10-17T01:06:27.210935226+00:00"
status: "done"
title: "What earns merits here"
updated_at: "2026-10-17T01:06:27.210935226+00:00"
---

## Goal

Suggest activities per system state and type (bounty hunting, data, mining and selling in the same system, power commodities, holoscreen hacking, ship scans, …).

## Done when

For each state in the rules table, a fixture system gets the expected suggestions.

## Context

Rules table from community guides (Inara's Powerplay 2.0 guide). Version it as data, since Frontier rebalances often.

## Close Reason

Done: versioned activity rules (data/catalog/powerplay-activities.json, wiki + refcard, credited in NOTICE.md), SystemPower::suggestions, 'What earns merits here' panel on the Powerplay page, voice status.merits_here. Done-when fixture test covers every state in the table. Uncertain rules marked; in-game check filed as astromech-c1sq (needs-human).
