---
created_at: "2026-10-16T18:12:14.692273628+00:00"
id: "astromech-3njr"
issue_type: "task"
labels:
- "size-l"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-fgv5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ship detail with live build stats"
updated_at: "2026-10-16T18:12:14.692273628+00:00"
---

## Goal

The ship detail shows the full stats from the ship calculations, both current and planned, next to the build plan (blueprint, grade and experimental per module). Changing the plan updates the planned stats and the difference at once. This is how the user answers "charge-enhanced or weapon-focused?" by watching distributor drain against recharge, "mass manager or deep charge?" by watching jump range, and "how many thermal shield boosters?" by watching thermal resistance and shield strength.

## Done when

Component tests with a fixture ship show that switching the distributor experimental, the FSD blueprint and the number of thermal-resistant boosters each changes the planned stat, and the difference against current, to the values the calculation tests expect. The panels for materials, trades, farming and the engineer trip still work as they do on the Builds page today.

## Scope

Engineering changes only. Swapping in a different module, and saving more than one build per ship, are out of scope (astromech-xw9t).

## Context

The plan model is in `app/src/lib/engineering.ts` and `crates/astromech-engineering/src/plan.rs`.
