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
closed_at: "2026-10-16T20:12:29.826641362+00:00"
status: "done"
title: "Prospector callouts"
updated_at: "2026-10-16T20:12:29.826641362+00:00"
---

## Goal

On `ProspectedAsteroid` (materials with proportions, `Content`, `MotherlodeMaterial`, `Remaining`), speak only rocks worth mining: a per-ore threshold (e.g. platinum ≥ 20%, painite ≥ 25%, default "anything high-value above N%") and every motherlode ("Motherlode. Void opals."). Stay quiet on the rest, or play a short low tone. Then show refined count and tons per hour in the session view.

## Done when

Replay tests: a rock above threshold gets one short line naming ore and percentage; a rock below gets nothing (or the tone); a motherlode always gets a line. Thresholds are editable per ore in Settings. A mining fixture gives the expected refined count and tons per hour.

## Context

The most useful callout in practice (Sumeet, 15 Oct 2026). Value order comes from the mining sale advisor once it exists; start with a static order.

## Notes

- 2026-10-16 [note] Built in the commit 'Prospector callouts (astromech-omi1)'. Thresholds are stored in astromech-store (key callouts.prospector.thresholds), not astromech.toml: this branch has no config write path. Settings → Prospector callouts edits them through the core (prospector_thresholds / prospector_set); the page is checked by lint/svelte-check/build and a parseThreshold unit test, not seen running (it needs the core). Below-threshold rocks are silent (no tone). The refined count and tons per hour are computed and fixture-tested (astromech_gamestate::mining) but not shown in the window: follow-up astromech-p9r4.

## Close Reason

Replay tests: above threshold one short line, below nothing, motherlode always; thresholds editable per ore in Settings (stored in astromech-store); mining fixture gives refined count and tons per hour.
