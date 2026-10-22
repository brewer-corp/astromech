---
created_at: "2026-10-22T21:06:57.006610647+00:00"
id: "astromech-zftk"
issue_type: "bug"
labels:
- "size-s"
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
status: "todo"
title: "Journal catch-up: lines the game writes during the startup replay count as history"
updated_at: "2026-10-22T21:06:57.006610647+00:00"
---

## Goal

Lines the game writes to its current journal while astromech replays at startup are treated as live (sent and spoken), so no live line is missed in that window.

## Done when

A pipeline test that appends to the game's current journal, or creates a new journal, while catch_up is replaying sees those lines handled as live, and history written before startup is still not sent.

## Context

Review finding R3 (astromech-dsrp) on feature/astromech-g3qt: catch_up (crates/astromech-core/src/pipeline.rs) measures startup lengths after replaying the followed journal. Measuring first narrows the window but changes EDDN behaviour during catch-up, so it needs its own look.
