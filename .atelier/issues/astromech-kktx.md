---
created_at: "2026-10-17T06:02:43.435902660+00:00"
id: "astromech-kktx"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-kktx
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
closed_at: "2026-10-17T06:17:09.062161200+00:00"
status: "done"
title: "Remote cache: replacing the v0.1.5 table can race between core and app window"
updated_at: "2026-10-17T06:17:09.062161200+00:00"
---

## Goal

The drop of the old v0.1.5 remote_cache table in crates/astromech-data/src/cache.rs isn't in a transaction; if the core and the app window open an old file at the same moment the second drop can fail and that process falls back to an in-memory cache for the run (finding astromech-fkg8 on astromech-zs2q). Do the replacement in BEGIN IMMEDIATE and re-check the layout inside it.

## Done when

A test with two connections opening an old-layout cache at once leaves both on the file-backed cache.

## Close Reason

Replacement runs in BEGIN IMMEDIATE with a layout re-check; two-opener test passes
