---
created_at: "2026-10-15T20:36:13.648020132+00:00"
id: "astromech-94j4"
issue_type: "feature"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-94j4
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-rk4s"
  children:
  - kind: "issue"
    id: "astromech-4fe3"
  - kind: "issue"
    id: "astromech-dqmv"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-5sbu"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Spansh plotters in the app"
updated_at: "2026-10-17T18:34:44.670942378+00:00"
---

## Goal

The user plots neutron and exact routes inside astromech, using the real jump range from the current loadout, and the route goes straight to the follower. This replaces Spansh's plotter pages.

## Done when

Neutron and exact routes, plotted from recorded Spansh responses with the jump range from the current loadout, load into the route follower. Independent review. The game check (astromech-rk4s) runs after this merges.

## Scope

Already built: the route follower, clipboard and galaxy-map paste ([PLOTTING.md](docs/PLOTTING.md) layer 1, branch `route-clipboard`).

## Unknowns

Spansh's loadout input format for the exact plotter.
