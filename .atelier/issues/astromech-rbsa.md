---
created_at: "2026-10-16T19:29:46.515122148+00:00"
id: "astromech-rbsa"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-z4jb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Label the kept experimental when a plan keeps the blueprint"
updated_at: "2026-10-16T19:29:46.515122148+00:00"
---

## Goal

On a same-blueprint target, choosing experimental "None" in the ship detail still keeps the fitted experimental (you can't remove one in game), so the planned stats include it while the dropdown says None.

## Done when

On a same-blueprint target the empty option reads "Keep <current experimental>" (or is removed); a component test shows it.

## Context

`crates/astromech-engineering/src/plan.rs` (`planned_modules`), `app/src/modules/engineering/Build.svelte`. Found in review of astromech-3njr (R2).
