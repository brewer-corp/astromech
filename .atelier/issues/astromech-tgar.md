---
created_at: "2026-10-17T16:28:50.524437750+00:00"
id: "astromech-tgar"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ship stats: count charge time in sustained DPS"
updated_at: "2026-10-17T17:01:41.186636222+00:00"
---

## Goal

Sustained DPS and the sustained factor count charge time, so sustained never exceeds burst, as decided in astromech-bnnh.

## Done when

- The reload cycle includes the charge time of the last burst; sustained factor is at most 1 for every weapon in `data/catalog`, checked by a test over the catalogue.
- The Fer-de-Lance fixture's sustained DPS is at or below its burst DPS, with the expected value worked by hand in the test.
- Weapons without charge keep their current, Coriolis-identical numbers.

## Context

`Module::sustained_factor` in `crates/astromech-ship/src/module.rs` ports Coriolis's `getSustainedFactor`: `clip / ((clip - burst) / rof + overhead + reload)`. The last burst's charge time is not in that cycle, so charged weapons with small clips run faster sustained than burst (the Fer-de-Lance fixture: DPS 124.5, sustained 136.2 with rail guns). `rof()` already counts charge per shot. Time to drain is unaffected. The doc comment and the ship detail (astromech-3njr) should say the figure departs from Coriolis for charged weapons.
