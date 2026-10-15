---
created_at: "2026-10-15T20:36:13.661534850+00:00"
id: "astromech-sd56"
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
title: "Sell my cargo"
updated_at: "2026-10-15T20:36:13.661534850+00:00"
---

## Goal

From `Cargo.json`, find the best station for the whole load and the best per commodity.

## Done when

For a fixture cargo of three commodities, the whole-load answer maximises total proceeds within demand caps, and per-commodity answers match the single-commodity sell lookup.

## Context

The most common trade question. Demand caps matter for mined goods.
