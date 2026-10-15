---
created_at: "2026-10-15T20:30:24.614337926+00:00"
id: "astromech-is8y"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-56w8"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Bio signals and species prediction"
updated_at: "2026-10-15T20:30:24.614337926+00:00"
---

## Goal

Genus list per body, and species prediction from atmosphere, temperature, gravity and star type (BioScan-like), with min-max value.

## Done when

For fixture bodies with known species, the prediction includes the real species and its value range.

## Context

Counts from `FSSBodySignals` (FSS); genus list from `SAASignalsFound.Genuses` (DSS). Prediction tables are community data; check the licence for each.
