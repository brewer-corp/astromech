---
created_at: "2026-10-17T03:05:46.077220570+00:00"
id: "astromech-qqa9"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show the project-commodity count for the docked market"
updated_at: "2026-10-17T03:05:46.077220570+00:00"
---

## Goal

Show the count of project commodities a market sells on the app, not only by voice (astromech-kv9k R1).

## Done when

- While docked with a fresh `Market.json`, the Colonisation page (or the Station page) shows how many commodities still needed by an active project this market has in stock.
- A component test covers it.
- A pipeline-level test drives the `Market` branch of `load_companions` (crates/astromech-core/src/pipeline.rs) through the real pipeline, not just `Run::market` (R2).
