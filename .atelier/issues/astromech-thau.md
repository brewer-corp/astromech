---
created_at: "2026-10-15T20:36:13.660309886+00:00"
id: "astromech-thau"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-uxyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:23:24.966911365+00:00"
status: "done"
title: "EDDN outfitting/2 and shipyard/2"
updated_at: "2026-10-16T19:23:24.966911365+00:00"
---

## Goal

Send `outfitting/2` and `shipyard/2` from `Outfitting.json` and `Shipyard.json`.

## Done when

Fixture files produce messages that validate against both live schemas.

## Notes

- 2026-10-16 [note] Spec read: EDCD/EDDN live 4ad669bb (16 Oct 2026), outfitting/shipyard READMEs + v2.0 schemas. Journal Outfitting.json items have no sku, so the README's sku rule (PowerPlay and tech-broker items) can't filter them from the journal; the filter is implemented for items that do carry sku, and PowerPlay/tech-broker modules with Hpt_/Int_ names will still be sent from journal data. Name prefix check uses exactly the schema pattern's cases (Hpt_/hpt_/Int_/int_/_Armour_/_armour_). An empty modules or ships list after filtering isn't sent (schema minItems 1). Ships use PriceList[].ShipType; allowCobraMkIV only when the file has AllowCobraMkIV.

## Close Reason

astromech_eddn::companion builds outfitting/2 and shipyard/2 from Outfitting.json and Shipyard.json per the live READMEs; fixtures validate against both live schemas (evidence recorded). Independent review pending (orchestrator).
