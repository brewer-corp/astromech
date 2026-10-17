---
created_at: "2026-10-15T20:36:13.665831070+00:00"
id: "astromech-ce4s"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T05:10:05.140031752+00:00"
status: "done"
title: "Landings per body and first-footfall hint"
updated_at: "2026-10-17T05:10:05.140031752+00:00"
---

## Goal

From `Touchdown`/`Disembark` with coordinates in the history, show "you've landed on A 2 before" in the system view and the arrival briefing. Add a "first footfall likely" hint for a landable body the user discovered (`WasDiscovered: false`) in an unpopulated system.

## Done when

History fixtures: a body landed on twice shows 2; an undiscovered landable body in an unpopulated system shows the hint.

## Context

Covers EDCoPilot's "number of landings on landable bodies" (inferred to come from its own journal history). True first footfall isn't in the journal.

## Close Reason

Landings per body from Touchdown/Disembark in the history, in the system view and the arrival briefing; first-footfall-likely hint.
