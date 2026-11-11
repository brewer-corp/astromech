---
schema: atelier.item
schema_version: 3
id: astromech-rbsa
title: Label the kept experimental when a plan keeps the blueprint
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T19:29:46.515122148Z
---

## Goal

On a same-blueprint target, choosing experimental "None" in the ship detail still keeps the fitted experimental (you can't remove one in game), so the planned stats include it while the dropdown says None.

## Done when

- [ ] On a same-blueprint target the empty option reads "Keep <current experimental>" (or is removed); a component test shows it.

## Context

`crates/astromech-engineering/src/plan.rs` (`planned_modules`), `app/src/modules/engineering/Build.svelte`. Found in review of astromech-3njr (R2).

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-z4jb (Ships page with live build stats)
