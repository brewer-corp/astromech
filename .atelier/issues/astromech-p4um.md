---
created_at: "2026-10-17T17:23:49.179329542+00:00"
id: "astromech-p4um"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-p4um
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Share JournalTail between the core pipeline and astromech-journal"
updated_at: "2026-10-17T18:31:57.418600868+00:00"
---

## Goal

Move `JournalTail` (crates/astromech-core/src/pipeline.rs, read_new) into astromech-journal and use it from `JournalWatch` (which re-implements offset tracking and partial-line handling) and from `journal_version`, so one implementation reads the open journal.

## Done when

`JournalWatch` and the core pipeline share one tail reader, with the existing tests (crates/astromech-journal/tests/live_watch.rs, pipeline tests) passing.

## Context

Found in review of the astromech-ztrb PR. astromech-mh3m and astromech-ogq4 also change the Tailer in pipeline.rs, so do this after they merge.
