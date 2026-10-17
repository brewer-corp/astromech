---
created_at: "2026-10-17T06:25:51.321902969+00:00"
id: "astromech-qtiw"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-qtiw
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
title: "Remote cache: two processes opening a new cache file at once leave one in memory"
updated_at: "2026-10-17T06:31:00.581514751+00:00"
---

## Goal

Two processes opening a brand-new (or non-WAL) remote cache file at once leave one on the in-memory cache: the WAL switch fails with "database is locked" despite the busy timeout (reproduced 4 of 4; finding astromech-uo5a on astromech-kktx). It can happen on first launch if the core and the app window start together. Retry the WAL switch (SQLite doesn't run the busy handler for it; astromech-store's enable_wal from astromech-n1ij does this).

## Done when

The two-opener test gains a fresh-file case and both openers end on the file-backed cache.
