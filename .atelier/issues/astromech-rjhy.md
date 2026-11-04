---
created_at: "2026-11-04T22:37:46.901442567+00:00"
id: "astromech-rjhy"
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
  - kind: "review"
    id: "astromech-0576"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal startup lengths: measure recent journals through a handle"
updated_at: "2026-11-04T22:37:46.901442567+00:00"
---

## Goal

Lines the game wrote to its current journal before astromech started are never live, even when that journal is neither the latest by name nor the one the catch-up follows.

## Done when

Every journal whose name stamp falls within the last day is measured through an open handle at startup (not the directory listing, which lags on Windows for a file the game holds open), with a pipeline test where a later-stamped foreign journal is in the folder and the game's journal is measured by handle.

## Context

Review finding R3 (astromech-0576) on bug/astromech-zftk, pre-existing: crates/astromech-core/src/pipeline.rs:705-712.
