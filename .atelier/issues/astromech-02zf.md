---
created_at: "2026-10-17T17:29:23.242363342+00:00"
id: "astromech-02zf"
issue_type: "task"
labels:
- "docs-found"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-wvet"
    type: "duplicates"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T17:41:24.131518933+00:00"
status: "superseded"
title: "Mining lookups (where to sell ore, nearest hotspot) have no UI or voice entry"
updated_at: "2026-10-17T17:41:24.131518933+00:00"
---

## Goal

crates/astromech-market/src/mining.rs has sale() (where to sell a load of one ore, stations capped by demand) and hotspots() (nearest ring hotspot of an ore, Spansh), but nothing in the app, the voice grammar or the control API calls them. The Mining page shows only the current run. Decide: wire them up (Mining page, "where can I sell my painite" already works for any commodity) or remove the dead code.

## Done when

Either the mining lookups are reachable by a player and in the guide (docs/guide/mining.md), or the code is gone. Found while writing the user guide.

## Close Reason

Duplicate of astromech-wvet (mining lookups).
