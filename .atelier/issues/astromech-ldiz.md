---
created_at: "2026-10-17T17:28:29.762459570+00:00"
id: "astromech-ldiz"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Tie the shell's Engineering watch list to build_journal_state"
updated_at: "2026-10-17T17:28:29.762459570+00:00"
---

## Goal

Make it impossible for the Engineering page's watch list (EngineeringCache::default in app/src-tauri/src/main.rs) to drift from what build_journal_state reads.

## Done when

Either the watch list and build_journal_state are driven by the same constants, or a test asserts every event a reader handles is in the watch list.

## Context

Found in review of the astromech-ztrb PR (finding astromech-7xex). A reader added without updating the list would silently stop page updates for that event.
