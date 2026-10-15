---
created_at: "2026-10-15T20:21:44.852963798+00:00"
id: "astromech-6ftp"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-5lbb"
  - kind: "issue"
    id: "astromech-thko"
  - kind: "issue"
    id: "astromech-vsfe"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Spansh plotters in the app"
updated_at: "2026-10-15T20:21:45.008850044+00:00"
---

## Scope

Already built: the route follower, clipboard and galaxy-map paste ([PLOTTING.md](docs/PLOTTING.md) layer 1, branch `route-clipboard`).

## Outcome

The user plots neutron and exact routes inside astromech, using the real jump range from the current loadout, and the route goes straight to the follower. This replaces Spansh's plotter pages.

## Delivers

S1

## Unknowns

Spansh's loadout input format for the exact plotter.
