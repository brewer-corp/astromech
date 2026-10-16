---
created_at: "2026-10-15T20:36:13.659069834+00:00"
id: "astromech-0zcu"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-6920"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:15:18.925711909+00:00"
status: "done"
title: "Being-scanned callout"
updated_at: "2026-10-16T20:15:18.925711909+00:00"
---

## Goal

On `Scanned` (cargo, crime, cabin, data), say a short line such as "Cargo scan." and what it means for the cargo aboard: "Cargo scan. Stolen goods aboard." (stolen flag in `Cargo.json`, or goods on this system's prohibited list).

## Done when

Replay tests: each scan type gives its line; the stolen-goods suffix appears only when the cargo fixture has stolen or prohibited goods.

## Context

The HUD warning is small and easy to miss in a fight, so this one is spoken (Sumeet, 15 Oct 2026). `Scanned` doesn't say who scanned you, and the last `ShipTargeted` is only a guess, so don't name the ship. Interdiction, hull, shields and heat stay unspoken.

## Notes

- 2026-10-16 [note] Stolen goods come from Cargo.json (Stolen per item). Prohibited goods are not checked: no local data source (Market.json has no prohibited flag); backlog astromech-p8ud. The suffix is on cargo scans only, since only a cargo scan finds cargo.

## Close Reason

Replay tests: each scan type gives its line; the stolen-goods suffix appears only with stolen cargo in the fixture. Prohibited goods split to astromech-p8ud (no data source).
