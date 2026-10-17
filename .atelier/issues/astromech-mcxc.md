---
created_at: "2026-10-15T20:36:13.662084535+00:00"
id: "astromech-mcxc"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mcxc
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
closed_at: "2026-10-17T02:54:02.253762882+00:00"
status: "done"
title: "Mining sale advisor and hotspot finder"
updated_at: "2026-10-17T02:54:02.253762882+00:00"
---

## Goal

Best nearby sale for LTDs, platinum, painite and the rest, with demand (caps matter for mining), plus "hotspot near me" from Spansh body search on ring signals. Its value order also ranks prospector callouts.

## Done when

Recorded-response tests return the best sale respecting demand, and the nearest ring with a hotspot for a chosen ore.

## Close Reason

Mining sale and ring hotspot lookups in astromech-market::mining with recorded-response tests; ore order shared with prospector callouts; app UI split to astromech-wvet.
