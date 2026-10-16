---
created_at: "2026-10-15T20:36:13.661534850+00:00"
id: "astromech-sd56"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-1per"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:49:54.495905604+00:00"
status: "done"
title: "Sell my cargo"
updated_at: "2026-10-16T20:49:54.495905604+00:00"
---

## Goal

From `Cargo.json`, find the best station for the whole load and the best per commodity.

## Done when

For a fixture cargo of three commodities, the whole-load answer maximises total proceeds within demand caps, and per-commodity answers match the single-commodity sell lookup.

## Context

The most common trade question. Demand caps matter for mined goods.

## Notes

- 2026-10-16 [note] Built in b7f8ebf: astromech_market::cargo (sellable, whole_load, plan, plan_search, from_journal_dir), journal CargoItem with symbols, Trade > My cargo, shell command trade_cargo, voice trade.sell_cargo answered in astromech-core with the trade.cargo filters. Proof: evidence for cargo plan, voice phrases, full checks, UI (all pass at b7f8ebf). Open: shell compiled only on Windows CI; real Cargo.json and live Ardent only in the game check astromech-1per; independent review outstanding.

## Close Reason

Fixture hold of three commodities: the whole-load answer maximises total proceeds within demand caps and per-commodity answers match the single-commodity sell lookup (recorded-response tests), voice phrase tests, UI check, full checks all pass at b7f8ebf. Independent review is the parent's gate.
