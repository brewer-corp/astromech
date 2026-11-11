---
schema: atelier.item
schema_version: 3
id: astromech-qqa9
title: Show the project-commodity count for the docked market
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T03:05:46.077220570Z
---

## Goal

Show the count of project commodities a market sells on the app, not only by voice (astromech-kv9k R1).

## Done when

- [ ] - While docked with a fresh `Market.json`, the Colonisation page (or the Station page) shows how many commodities still needed by an active project this market has in stock. - A component test covers it. - A pipeline-level test drives the `Market` branch of `load_companions` (crates/astromech-core/src/pipeline.rs) through the real pipeline, not just `Run::market` (R2).
