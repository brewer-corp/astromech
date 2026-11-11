---
schema: atelier.item
schema_version: 3
id: astromech-os70
title: 'Linked places: engine, recipes, Find near this, Find along route'
status: ready
mission: astromech-7q0z
labels:
- size-l
- feature
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T06:28:18.541440404Z
trivial: null
caused_by: null
created_at: 2026-11-05T06:27:49.261153493Z
---

## Goal

Answer linked-place questions ("an HGE system near a material trader", "a Platinum hotspot near a good buyer") from Places: build the linked-place engine spike astromech-yn6y proved, put its two recipes and "Find near this" in Places, and add Find along route ranked by detour (decision astromech-bi94: built as linked places from the start; chaining, then recipes). Follows astromech-51ue, which builds Places, the route strip and the result actions.

## Done when

- [ ] On the Linux build, shown by the children's tests and `scripts/ui` screenshots, an independent review of the branch, and the validation child's receipts: - The engine runs the spike's two recipes over recorded responses within the spike's budget (at most 2 live legs and about 4 requests per query) and refuses plans over it. - Both recipes run from Places with their slots, and "Find near this" on a result row opens a finder pinned to that place. - Find along route ranks places by the jumps they add to the followed route, and Add stop inserts at the leg found. - Each new finder has a voice query, and the guide describes what shipped.

## Unknowns

- Corridor queries for Find along route have not been measured (astromech-wla4).
- Whether the local station, ring and hotspot file (astromech-ibmy) is needed depends on measured request volume and on the Spansh maintainer's answer (astromech-5rm2, asked in parallel; nothing here waits on it).

## Background

Spike astromech-yn6y's close reason sets the engine's shape, budget and which legs are live. The finders and row grammar come from astromech-51ue's children (astromech-nxz9, astromech-jp36, astromech-rz0f, astromech-1d92); the blocking links sit on the children that need them, so the engine (astromech-xlbz) can start now. Design choices: astromech-g8d5.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-51ue (Galaxy rework: route strip, Places finders, one search box)
- derived_from: astromech-bi94 (Galaxy rework: direction for routes and place-finding)
