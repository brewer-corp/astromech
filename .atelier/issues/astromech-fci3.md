---
created_at: "2026-11-05T16:10:44.174807543+00:00"
id: "astromech-fci3"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-51ue"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find or plot: a full system name entered at once plots; V1-DESIGN Exploration row"
updated_at: "2026-11-05T16:10:44.174807543+00:00"
---

## Goal

A system name typed in full and entered at once in Find or plot always plots, and V1-DESIGN describes Exploration's links as shipped.

## Done when

- "Colonia" typed and entered before Spansh answers opens the plot panel, not the Colonisation page: Enter waits for the system answer while it is being asked, or only whole-label and finder-phrase matches count as sure (finding astromech-s55s). A PlotBar test enters "Colonia" at once; "Col" plus Enter still opens Colonisation once Spansh has answered.
- docs/V1-DESIGN.md's Exploration: System row (around line 197) says Where to sell links into the Services finder (finding astromech-d3vj).
- `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred findings from the final 51ue review (reviewer-fable-51ue-final).
