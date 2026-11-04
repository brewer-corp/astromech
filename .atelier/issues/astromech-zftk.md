---
created_at: "2026-10-22T21:06:57.006610647+00:00"
id: "astromech-zftk"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-zftk
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Journal catch-up: lines the game writes during the startup replay count as history"
updated_at: "2026-11-04T21:42:42.623942814+00:00"
---

## Goal

Lines the game writes to its current journal while astromech replays at startup are treated as live (sent and spoken), so no live line is missed in that window.

## Done when

A pipeline test that appends to the game's current journal, or creates a new journal, while catch_up is replaying sees those lines handled as live, and history written before startup is still not sent.

## Context

Review finding R3 (astromech-dsrp) on feature/astromech-g3qt: catch_up (crates/astromech-core/src/pipeline.rs) measures startup lengths after replaying the followed journal. Measuring first narrows the window but changes EDDN behaviour during catch-up, so it needs its own look.

## Notes

- 2026-11-04 [decision] Measure every journal's startup length before the catch-up reads anything (the followed one through an open handle, like the latest by name), and during the catch-up replay the followed journal only up to that length; what lies past it is left for the first read after caught_up, so it goes through the normal live path (EDDN, EDSM, callouts, companion files).
  Reason: Measuring first closes the window for the game's current journal and for a journal it starts during the replay. Reading the followed journal's remainder inside the catch-up would send it to EDDN (which uses self.live) while callouts are still off (caught_up false), so lines would be sent but not spoken; deferring it to the next read (at most 250 ms) gives one notion of live and needs no new state. The followed journal is measured through a handle because a lagging directory listing would make history live.
