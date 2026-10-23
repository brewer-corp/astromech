---
created_at: "2026-10-23T00:08:31.390679291+00:00"
id: "astromech-58a7"
issue_type: "task"
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
title: "Warn when stations list outfitting modules astromech doesn't know"
updated_at: "2026-10-23T00:08:31.390679291+00:00"
---

## Goal

After a game update adds modules, astromech's maintainers learn quickly that outfitting messages are being refused for unknown modules, instead of only a per-visit Diagnostics line.

## Done when

The real-journal harness reports unknown outfitting symbols as problems, and a scheduled or release-time check (or the release checklist) re-runs scripts/gen-eddn-outfitting.py against the latest FDevIDs and fails on new symbols; documented in CONTRIBUTING or the release notes process.

## Context

Review finding R3 (astromech-tbrk) on task/astromech-dlrb: crates/astromech-eddn/src/companion.rs:254 refuses a station's whole outfitting message when a module is in neither list.
