---
created_at: "2026-10-17T05:19:41.206256062+00:00"
id: "astromech-e8up"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-e8up
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
status: "in_progress"
title: "Exploration values: no discovery bonuses for bodies already mapped in the bubble"
updated_at: "2026-10-17T07:33:04.026280930+00:00"
---

## Goal

Bodies with WasDiscovered false but WasMapped true (the old populated bubble) still get the Odyssey 30% and the x2.6 first-discovery bonus in astromech-exploration's value estimate; EDDI gives them neither (finding astromech-qcdt on astromech-wu30).

## Done when

A value test for a WasDiscovered=false, WasMapped=true body matches EDDI's estimate.
