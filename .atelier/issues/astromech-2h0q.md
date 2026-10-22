---
created_at: "2026-10-16T20:58:31.495419709+00:00"
id: "astromech-2h0q"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ipb5"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Share one journal-growth check; history import length fallback"
updated_at: "2026-10-22T20:57:08.011218168+00:00"
---

## Goal

`journal_version` (the open-handle check for journal growth on Windows) is duplicated in astromech-core history.rs and astromech-materials view.rs; `History::import` now opens each file to read its length, dropping std::fs::metadata's fallback when a file can't be opened; ARCHITECTURE.md's "Crates not listed" still names history; a no-op `version = now` in the follower wait loop.

## Done when

One `journal_version` in astromech-journal used by both; import falls back to metadata if the open fails; the doc and the no-op are fixed.

## Context

Found in review of the astromech-ipb5 Windows fix (R1–R3).
