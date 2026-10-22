---
created_at: "2026-10-22T19:30:44.588769708+00:00"
id: "astromech-zrbk"
issue_type: "feature"
labels:
- "size-l"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-1842"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Neutron routes that plan fuel and check sparse gaps"
updated_at: "2026-10-22T19:30:44.588769708+00:00"
---

## Goal

Local neutron routes say where to refuel, since neutron stars can't be scooped, and don't promise legs that can't be flown: where a gap between stops is in sparse space, or at either end of the route, the plotter checks the real stars there before trusting its jump estimate.

## Done when

Provisional, to make concrete when planned:
- A route lists a scoopable star near each stop where the tank would otherwise run low, with the stop labelled "fuel", for the ship's tank, reservoir and fuel use per jump (boosted jumps included).
- Gaps the plotter flags as doubtful (low local star density, far off the plane) are checked against the systems around them, fetched on demand into `galaxy.db`; a gap with no flyable chain at the ship's range is not used.
- The start and destination legs are worked out from the systems around them, not only estimated.
- Tests on recorded neighbourhoods, and the guide's limit lines for fuel and sparse gaps go.

## Unknowns

- Fuel use of a boosted jump: Spansh's exact plotter models it (`spansh-exact-*` fixtures); confirm the formula before relying on it.
- Which source gives scoopable stars (KGBFOAM primaries) around a point cheaply: Spansh's systems search by primary star class and distance, or the EDSM sphere fill-in (capped at about 100 ly).
- How to mark a gap as doubtful: star density from the local star map, distance from the galactic plane, or both.

## Background

Follows astromech-1842, which estimates every gap and plans no fuel, as Spansh's neutron plotter does. Discussed with Sumeet on 22 Oct 2026: normal stars are fetched on demand for the neighbourhoods that need them, not shipped.
