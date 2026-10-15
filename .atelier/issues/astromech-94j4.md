---
created_at: "2026-10-15T20:36:13.648020132+00:00"
id: "astromech-94j4"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-4f6x"
  - kind: "issue"
    id: "astromech-4fe3"
  - kind: "issue"
    id: "astromech-dqmv"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Spansh plotters in the app"
updated_at: "2026-10-15T20:36:13.798453258+00:00"
---

## Scope

Already built: the route follower, clipboard and galaxy-map paste ([PLOTTING.md](docs/PLOTTING.md) layer 1, branch `route-clipboard`).

## Outcome

The user plots neutron and exact routes inside astromech, using the real jump range from the current loadout, and the route goes straight to the follower. This replaces Spansh's plotter pages.

## Unknowns

Spansh's loadout input format for the exact plotter.
