---
created_at: "2026-10-17T02:53:44.713252819+00:00"
id: "astromech-wvet"
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
closed_at: "2026-11-05T06:20:24.026069743+00:00"
status: "superseded"
title: "Mining page: sale and hotspot lookups in the app"
updated_at: "2026-11-05T06:20:24.026069743+00:00"
---

## Goal

A Mining section in the Commodities page (app/src/modules/trade): pick an ore and tonnes, show the best sale within demand caps and the nearest ring hotspots. Needs two Tauri commands in app/src-tauri/src/main.rs modelled on trade_cargo (search_origin), calling astromech_market::mining::sale and hotspots_search, a lib/trade.ts wrapper with dev mock data, and component tests. The ore choices come from astromech_journal::ore::VALUE_ORDER.

## Done when

The page shows the best sale and the hotspots for a chosen ore from recorded data, with component tests, and the commands type-check.

## Close Reason

Decision astromech-bi94 moves mining hotspots into Places as a finder; astromech-f7i0 (child of astromech-51ue) replaces this Commodities-page section. Ore sales are covered by the Sell finder, which shows demand.
