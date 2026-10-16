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
status: "in_progress"
title: "Use the journal's engineering Modifiers for current ship stats"
updated_at: "2026-10-16T20:59:11.818705974+00:00"
---

## Goal

astromech-ship rebuilds current stats from blueprint, grade and roll. Real journal Loadouts carry `Modifiers` with the module's actual rolled values, and Coriolis uses them when present. Legacy pre-3.0 modules show Quality 0 with their real values only in `Modifiers`, so current jump range can be off by 1–2 ly.

## Done when

Loadout parsing keeps `Modifiers`; current stats use them (or flag the module approximate when they disagree with blueprint and roll); planned stats keep using blueprint and roll. A fixture with a legacy module shows the journal value.

## Context

`crates/astromech-ship/src/lib.rs`, `crates/astromech-journal/src/lib.rs` (Loadout). Found in review of astromech-ojwr (R1). Open question for Sumeet: does the journal report pre-engineered V1 SCO/FSD modules at Quality 1.0 or 0?
