---
schema: atelier.item
schema_version: 3
id: astromech-xlbz
title: Linked-place query engine with the spike's two recipes
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
  at: 2026-11-05T06:27:50.016102468Z
trivial: null
caused_by: null
created_at: 2026-11-05T06:19:45.521881351Z
---

## Goal

Build the linked-place query engine the spike astromech-yn6y proved: a query is a plan of legs, each a kind of place with conditions and optionally "within N ly of" another leg, from an origin and radius, ranked by a goal. Each leg is fetched once as a set around the origin and the sets are joined locally. Recipes, "find near this" and Find along route build on it (decision astromech-bi94: built as linked places from the start).

## Done when

- [ ] - A Rust module exposes a plan type ({kind, conditions, near: (leg, max_ly)} legs, origin, radius, goal as ordered score terms) and runs it through the existing rate-limited `astromech_data::Client`: one set per leg within radius plus link of the origin, the join and re-check done locally, fan-out only when one side has no bound (chosen with Spansh's `count` probe), and a refusal when a plan would exceed the budget (at most 2 live legs and about 4 requests). - Leg kinds for this child: systems by faction state, allegiance and population (Spansh systems search); stations by service or trader kind (Spansh stations search); rings by hotspot mineral, count and reserve (Spansh bodies search); buyers of a commodity above a price (Ardent nearby imports); and permit-locked systems as an exclusion set. - Every goal scores arrival distance and data age. An empty answer widens the search once and then names the nearest match. - The spike's two recipes run as tests over recorded responses committed as fixtures (trimmed to what the tests need): HGE-likely system within 20 ly of a manufactured trader near Sol, Federation; Platinum hotspot ring with a buyer at 250k cr or more within 30 ly. Tests assert the request count per query, the top answers, and the arrival and age scoring; one test pins the budget refusal. - `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` and `python3 scripts/check-deps.py` pass.

## Scope

Rust only; no UI. Where it lives: `astromech-market` already depends on `astromech-galaxy` and `astromech-data`; a new crate that depends on both is the alternative. Record the choice as a decision note.

## Context

- Spike close reason (astromech-yn6y): request counts, candidate sizes, Ardent's quirks (ignores `minLandingPadSize`, rounds distance, leaves out the searched system), the 94 kB station records, scoring arrival distance. Prototype: `spikes/q1-linked-places/` (its `src/` shows the requests; its cache is gitignored, so record fresh fixtures with the existing recording transport).
- Prices come from Ardent, not Spansh's market search (weeks stale).

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-os70 (Linked places: engine, recipes, Find near this, Find along route)
