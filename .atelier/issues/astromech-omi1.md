---
created_at: "2026-10-15T20:36:13.659338625+00:00"
id: "astromech-omi1"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-6920"
  - kind: "issue"
    id: "astromech-xes6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Prospector callouts"
updated_at: "2026-10-16T20:07:37.040402694+00:00"
---

## Goal

On `ProspectedAsteroid` (materials with proportions, `Content`, `MotherlodeMaterial`, `Remaining`), speak only rocks worth mining: a per-ore threshold (e.g. platinum ≥ 20%, painite ≥ 25%, default "anything high-value above N%") and every motherlode ("Motherlode. Void opals."). Stay quiet on the rest, or play a short low tone. Then show refined count and tons per hour in the session view.

## Done when

Replay tests: a rock above threshold gets one short line naming ore and percentage; a rock below gets nothing (or the tone); a motherlode always gets a line. Thresholds are editable per ore in Settings. A mining fixture gives the expected refined count and tons per hour.

## Context

The most useful callout in practice (Sumeet, 15 Oct 2026). Value order comes from the mining sale advisor once it exists; start with a static order.
