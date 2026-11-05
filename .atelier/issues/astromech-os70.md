---
created_at: "2026-11-05T06:27:49.261153493+00:00"
id: "astromech-os70"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-cu7d"
  - kind: "issue"
    id: "astromech-ibmy"
  - kind: "issue"
    id: "astromech-sgwi"
  - kind: "issue"
    id: "astromech-wla4"
  - kind: "issue"
    id: "astromech-xlbz"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-51ue"
    type: "related"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Linked places: engine, recipes, Find near this, Find along route"
updated_at: "2026-11-05T06:28:18.541440404+00:00"
---

## Goal

Answer linked-place questions ("an HGE system near a material trader", "a Platinum hotspot near a good buyer") from Places: build the linked-place engine spike astromech-yn6y proved, put its two recipes and "Find near this" in Places, and add Find along route ranked by detour (decision astromech-bi94: built as linked places from the start; chaining, then recipes). Follows astromech-51ue, which builds Places, the route strip and the result actions.

## Done when

On the Linux build, shown by the children's tests and `scripts/ui` screenshots, an independent review of the branch, and the validation child's receipts:

- The engine runs the spike's two recipes over recorded responses within the spike's budget (at most 2 live legs and about 4 requests per query) and refuses plans over it.
- Both recipes run from Places with their slots, and "Find near this" on a result row opens a finder pinned to that place.
- Find along route ranks places by the jumps they add to the followed route, and Add stop inserts at the leg found.
- Each new finder has a voice query, and the guide describes what shipped.

## Unknowns

- Corridor queries for Find along route have not been measured (astromech-wla4).
- Whether the local station, ring and hotspot file (astromech-ibmy) is needed depends on measured request volume and on the Spansh maintainer's answer (astromech-5rm2, asked in parallel; nothing here waits on it).

## Background

Spike astromech-yn6y's close reason sets the engine's shape, budget and which legs are live. The finders and row grammar come from astromech-51ue's children (astromech-nxz9, astromech-jp36, astromech-rz0f, astromech-1d92); the blocking links sit on the children that need them, so the engine (astromech-xlbz) can start now. Design choices: astromech-g8d5.
