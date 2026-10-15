---
created_at: "2026-10-15T20:21:44.904023787+00:00"
id: "astromech-vsfe"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-5lbb"
  - kind: "issue"
    id: "astromech-thko"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Core ship stats from the loadout"
updated_at: "2026-10-15T20:21:44.904023787+00:00"
---

## Goal

Laden and unladen jump range, max range, cargo, fuel, power use vs capacity, and rebuy. The journal `Loadout` already gives `MaxJumpRange`, `UnladenMass`, `CargoCapacity` and `Rebuy`.

## Done when

A fixture loadout gives stats matching the in-game values recorded with it.

## Context

Mostly journal numbers (partly built). Lets the plotters use the real range.
