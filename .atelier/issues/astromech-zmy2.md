---
created_at: "2026-10-15T20:21:44.884979795+00:00"
id: "astromech-zmy2"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Being-scanned callout"
updated_at: "2026-10-15T20:21:44.884979795+00:00"
---

## Goal

On `Scanned` (cargo, crime, cabin, data), say a short line such as "Cargo scan." and what it means for the cargo aboard: "Cargo scan. Stolen goods aboard." (stolen flag in `Cargo.json`, or goods on this system's prohibited list).

## Done when

Replay tests: each scan type gives its line; the stolen-goods suffix appears only when the cargo fixture has stolen or prohibited goods.

## Context

The HUD warning is small and easy to miss in a fight, so this one is spoken (Sumeet, 15 Oct 2026). `Scanned` doesn't say who scanned you, and the last `ShipTargeted` is only a guess, so don't name the ship. Interdiction, hull, shields and heat stay unspoken.
