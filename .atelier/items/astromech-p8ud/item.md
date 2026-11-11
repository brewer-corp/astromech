---
schema: atelier.item
schema_version: 3
id: astromech-p8ud
title: Prohibited goods in the scan callout
status: draft
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T20:14:13.392846043Z
---

## Goal

A cargo scan also warns when goods aboard are prohibited in the current system ("Cargo scan. Prohibited goods aboard."), not only when they are stolen.

## Done when

- [ ] Replay test: with a fixture cargo and a prohibited list for the system, the cargo scan line gets the prohibited suffix; without, it does not.

## Context

astromech-0zcu built the scan callout with the stolen-goods suffix only. The journal and its companion files (`Market.json` has no prohibited flag) do not say which goods are prohibited where; a source is needed first (Frontier cAPI market `prohibited`, or EDDN/Spansh market data).

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-cmhv (Reaction engine with prospector and scan callouts)
