---
created_at: "2026-10-16T19:06:19.433290279+00:00"
id: "astromech-fdyn"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-fdyn
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-z4jb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T21:13:41.193839547+00:00"
status: "done"
title: "Use the journal's engineering Modifiers for current ship stats"
updated_at: "2026-10-16T21:13:41.193839547+00:00"
---

## Goal

astromech-ship rebuilds current stats from blueprint, grade and roll. Real journal Loadouts carry `Modifiers` with the module's actual rolled values, and Coriolis uses them when present. Legacy pre-3.0 modules show Quality 0 with their real values only in `Modifiers`, so current jump range can be off by 1–2 ly.

## Done when

Loadout parsing keeps `Modifiers`; current stats use them (or flag the module approximate when they disagree with blueprint and roll); planned stats keep using blueprint and roll. A fixture with a legacy module shows the journal value.

## Context

`crates/astromech-ship/src/lib.rs`, `crates/astromech-journal/src/lib.rs` (Loadout). Found in review of astromech-ojwr (R1). Open question for Sumeet: does the journal report pre-engineered V1 SCO/FSD modules at Quality 1.0 or 0?

## Notes

- 2026-10-16 [note] Implemented on task/astromech-fdyn (e8f5ccb). Current stats use Modifiers when present (Coriolis _addModifications port, experimental share backed out); without them, blueprint+grade+roll. Planned: targeted modules drop Modifiers (blueprint+roll) unless the plan leaves them unchanged; untouched modules keep journal values so the plan diff shows only planned changes. EngineerCraft Modifiers replace the module's. Unknown labels go to unresolved as '<slot>: modifier <label>'; DamagePerSecond is skipped as derived (Coriolis reads it as damage, double-counting rate-of-fire changes). Added MaxFuelPerJump and CausticResistance, which Coriolis doesn't match. Deliberate differences from Coriolis: an empty Modifiers list falls back to Quality (Coriolis applies nothing); additive experimental back-out scales correctly (only Jitter, not shown). Open question (V1 pre-engineered SCO/FSD Quality 1.0 vs 0): with Modifiers the Quality value no longer matters as long as OriginalValue is the plain module's value; if the game writes the pre-engineered module's own value as OriginalValue, the factory engineering would be missed. Needs a real journal line to confirm.

## Close Reason

Loadout and EngineerCraft keep Modifiers; current stats use them (match Coriolis on engineered and legacy fixtures); planned stats use blueprint and roll for targeted modules; legacy fixture shows the journal value. Evidence astromech-iusl, astromech-nykk and the discrimination check.
