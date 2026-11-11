---
schema: atelier.item
schema_version: 3
id: astromech-wla4
title: Find along route, ranked by detour
status: draft
mission: astromech-7q0z
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-xlbz
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T06:19:45.850110825Z
---

## Goal

Add Find along route: a service, sale or other place ranked by the detour it adds to the followed route ("nearest refuel on route", "sell painite on the way"), with Add stop inserting it at the leg it was found on (decision astromech-bi94).

## Done when

- [ ] Provisional: a Places finder at `#/places/along-route` ranks results by added jumps (main value "+N jumps") against the followed route, for at least services and commodity sales; Add stop inserts at the found leg; recorded-response tests on a plotted route fixture; a voice query; screenshots; the guide.

## Unknowns

- How to fetch the corridor within the request budget: one sphere per N waypoints, a bounding search, or local stations once the local station file exists (astromech-ibmy). The spike named the shape (the A side is the route's corridor) but did not measure it.
- Waypoint coordinates for imported routes (the star map has them only near the bubble or where the commander has been).

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-os70 (Linked places: engine, recipes, Find near this, Find along route)
