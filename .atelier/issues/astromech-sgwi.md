---
created_at: "2026-11-05T06:19:45.685445791+00:00"
id: "astromech-sgwi"
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
    id: "astromech-5rm2"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xlbz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Recipes and Find near this in Places"
updated_at: "2026-11-05T06:19:45.685445791+00:00"
---

## Goal

Put linked places in front of the commander: the spike's two recipes as Places finders ("HGE system near a material trader", "Mining hotspot near a good buyer"), and "Find near this" on result rows, which pins a leg to that row and opens a finder for a second place near it (decision astromech-bi94: chaining, then saved recipes).

## Done when

Provisional: both recipes run from Places on the engine with their slots (allegiance and trader kind; mineral and minimum price) and show linked rows (the A place with its B place and the link distance); "Find near this" from a Material traders or Mining hotspots row opens a finder pinned to that place; component tests and recorded-response tests pass; screenshots; voice queries for both recipes; the guide describes them.

## Unknowns

- How a linked row fits the one-line result row (two places on one line, or the B place as a fact column).
- Which existing finders need to run through the engine for "Find near this"; the rest stay on their current lookups until they need it.
- Live request volume per query: the Spansh maintainer question (astromech-5rm2) gates shipping this.
