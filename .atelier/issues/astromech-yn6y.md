---
created_at: "2026-10-24T23:07:07.622893259+00:00"
id: "astromech-yn6y"
issue_type: "spike"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Compound place queries: test two linked-place recipes on real data"
updated_at: "2026-10-24T23:07:07.622893259+00:00"
---

## Goal

Find out whether a query engine for linked places (place A with conditions, within some distance of place B with conditions, ranked by one goal) is workable on the data astromech can reach, before the Places UI is designed around it.

## Done when

Two hard-coded recipes run end to end from a throwaway prototype under `spikes/`, against the live services through the existing rate-limited client or equivalent:

1. An HGE-likely system (Boom, Expansion or War/Civil War, at least 1 bn people, by allegiance) within N ly of a material trader of a chosen type, nearest to a given origin.
2. A ring with a hotspot for a chosen mineral within N ly of a station buying it above a price, ranked by price against extra jumps.

The findings record, for each: requests made, wall time at 1 request/s per service, the candidate-set sizes on each side, answer quality spot-checked against Inara/Spansh, and what failed. Also: which facts could come from a local static file built from the public Spansh dumps (stations, services, traders, rings, hotspots), its estimated size, and which must stay live (prices, faction states). Conclude with a go / no-go and the engine shape to build.

## Unknowns

- Whether Spansh's body/station search can filter rings by hotspot, and how complete hotspot data is.
- How fresh faction-state data is for HGE prediction.
- Whether searching from the rarer side (traders, hotspots) keeps candidate sets small enough.
