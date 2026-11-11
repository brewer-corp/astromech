---
schema: atelier.item
schema_version: 3
id: astromech-fci3
title: 'Find or plot: a full system name entered at once plots; V1-DESIGN Exploration row'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T16:10:44.174807543Z
trivial: null
caused_by: null
created_at: 2026-11-05T16:10:44.174807543Z
---

## Goal

A system name typed in full and entered at once in Find or plot always plots, and V1-DESIGN describes Exploration's links as shipped.

## Done when

- [ ] - "Colonia" typed and entered before Spansh answers opens the plot panel, not the Colonisation page: Enter waits for the system answer while it is being asked, or only whole-label and finder-phrase matches count as sure (finding astromech-s55s). A PlotBar test enters "Colonia" at once; "Col" plus Enter still opens Colonisation once Spansh has answered. - docs/V1-DESIGN.md's Exploration: System row (around line 197) says Where to sell links into the Services finder (finding astromech-d3vj). - `pnpm lint && pnpm check && pnpm test` pass.

## Context

Deferred findings from the final 51ue review (reviewer-fable-51ue-final).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-51ue (Galaxy rework: route strip, Places finders, one search box)
