---
created_at: "2026-11-05T06:19:45.850110825+00:00"
id: "astromech-wla4"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-1d92"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-rz0f"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xlbz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Find along route, ranked by detour"
updated_at: "2026-11-05T06:27:50.389323868+00:00"
---

## Goal

Add Find along route: a service, sale or other place ranked by the detour it adds to the followed route ("nearest refuel on route", "sell painite on the way"), with Add stop inserting it at the leg it was found on (decision astromech-bi94).

## Done when

Provisional: a Places finder at `#/places/along-route` ranks results by added jumps (main value "+N jumps") against the followed route, for at least services and commodity sales; Add stop inserts at the found leg; recorded-response tests on a plotted route fixture; a voice query; screenshots; the guide.

## Unknowns

- How to fetch the corridor within the request budget: one sphere per N waypoints, a bounding search, or local stations once the local station file exists (astromech-ibmy). The spike named the shape (the A side is the route's corridor) but did not measure it.
- Waypoint coordinates for imported routes (the star map has them only near the bubble or where the commander has been).
