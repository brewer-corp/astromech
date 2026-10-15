---
created_at: "2026-10-15T20:21:44.901115262+00:00"
id: "astromech-git0"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-y3bw"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Bio signals and species prediction"
updated_at: "2026-10-15T20:21:44.901115262+00:00"
---

## Goal

Genus list per body, and species prediction from atmosphere, temperature, gravity and star type (BioScan-like), with min-max value.

## Done when

For fixture bodies with known species, the prediction includes the real species and its value range.

## Context

Counts from `FSSBodySignals` (FSS); genus list from `SAASignalsFound.Genuses` (DSS). Prediction tables are community data; check the licence for each.
