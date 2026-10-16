---
created_at: "2026-10-15T20:36:13.665982432+00:00"
id: "astromech-4f6x"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-4fe3"
  - kind: "issue"
    id: "astromech-dqmv"
  - kind: "issue"
    id: "astromech-rk4s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:12:14.789814203+00:00"
status: "superseded"
title: "Core ship stats from the loadout"
updated_at: "2026-10-16T18:12:14.789814203+00:00"
---

## Goal

Laden and unladen jump range, max range, cargo, fuel, power use vs capacity, and rebuy. The journal `Loadout` already gives `MaxJumpRange`, `UnladenMass`, `CargoCapacity` and `Rebuy`.

## Done when

A fixture loadout gives stats matching the in-game values recorded with it.

## Context

Mostly journal numbers (partly built). Lets the plotters use the real range.

## Close Reason

Superseded 2026-10-16 by astromech-ojwr (Port Coriolis's ship calculations to Rust), which covers these stats and more.
