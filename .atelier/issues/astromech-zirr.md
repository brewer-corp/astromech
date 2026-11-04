---
created_at: "2026-11-04T22:42:52.491961752+00:00"
id: "astromech-zirr"
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
title: "Window config: ignore a config.path left by a core that is gone"
updated_at: "2026-11-04T22:42:52.491961752+00:00"
---

## Goal

A config.path left behind by a crashed, killed or older core doesn't make the window and the core it starts disagree on folders.

## Done when

- config.path records the publishing core's pid (or another liveness check), and the window ignores a note whose core is gone, with a test for a leftover note naming a deleted --config file (R1, astromech-69h9).
- docs/ARCHITECTURE.md no longer says the window can't see --config (R2, astromech-sb1n).

## Context

Deferred review findings R1 (astromech-69h9) and R2 (astromech-sb1n) on bug/astromech-f2k1: crates/astromech-store/src/paths.rs:108, docs/ARCHITECTURE.md:1774.
