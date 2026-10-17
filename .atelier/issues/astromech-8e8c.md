---
created_at: "2026-10-15T20:36:13.664928437+00:00"
id: "astromech-8e8c"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-a63r"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T04:56:59.767905766+00:00"
status: "done"
title: "Bio signals and species prediction"
updated_at: "2026-10-17T04:56:59.767905766+00:00"
---

## Goal

Genus list per body, and species prediction from atmosphere, temperature, gravity and star type (BioScan-like), with min-max value.

## Done when

For fixture bodies with known species, the prediction includes the real species and its value range.

## Context

Counts from `FSSBodySignals` (FSS); genus list from `SAASignalsFound.Genuses` (DSS). Prediction tables are community data; check the licence for each.

## Close Reason

Genus list per body (FSS counts, DSS genera) and species prediction with min-max value in astromech-exobiology, folded into GameState; data from the wiki and EDMC-ExplorerLite (MIT), credited in NOTICE.md.
