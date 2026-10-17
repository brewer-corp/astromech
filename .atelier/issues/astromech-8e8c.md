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
status: "in_progress"
title: "Bio signals and species prediction"
updated_at: "2026-10-17T04:48:27.229762762+00:00"
---

## Goal

Genus list per body, and species prediction from atmosphere, temperature, gravity and star type (BioScan-like), with min-max value.

## Done when

For fixture bodies with known species, the prediction includes the real species and its value range.

## Context

Counts from `FSSBodySignals` (FSS); genus list from `SAASignalsFound.Genuses` (DSS). Prediction tables are community data; check the licence for each.
