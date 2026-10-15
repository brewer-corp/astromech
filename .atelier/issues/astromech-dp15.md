---
created_at: "2026-10-15T20:21:44.885769769+00:00"
id: "astromech-dp15"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-x9g5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Prospector callouts"
updated_at: "2026-10-15T20:21:44.885769769+00:00"
---

## Goal

On `ProspectedAsteroid` (materials with proportions, `Content`, `MotherlodeMaterial`, `Remaining`), speak only rocks worth mining: a per-ore threshold (e.g. platinum ≥ 20%, painite ≥ 25%, default "anything high-value above N%") and every motherlode ("Motherlode. Void opals."). Stay quiet on the rest, or play a short low tone. Then show refined count and tons per hour in the session view.

## Done when

Replay tests: a rock above threshold gets one short line naming ore and percentage; a rock below gets nothing (or the tone); a motherlode always gets a line. Thresholds are editable per ore in Settings.

## Context

The most useful callout in practice (Sumeet, 15 Oct 2026). Value order comes from the mining sale advisor once it exists; start with a static order.
