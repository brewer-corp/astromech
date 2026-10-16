---
created_at: "2026-10-16T18:38:18.719728124+00:00"
id: "astromech-mh3m"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-txzq"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Core reads the journal only on file notifications"
updated_at: "2026-10-16T18:38:18.719728124+00:00"
---

## Goal

Make the core pick up journal lines promptly on Windows even when Status.json isn't changing, so voice-side game state doesn't lag.

## Done when

The core applies a line appended to the open journal within 1 s without any Status.json, Cargo.json or NavRoute.json change, and a test guards it.

## Context

`crates/astromech-core/src/pipeline.rs` reads the journal tail only when a notify event names a journal or a companion file. The 250 ms `recv_timeout` reads nothing. On Windows, ReadDirectoryChangesW reports size and last-write changes from the same lazily updated directory entry that delayed the Materials page (astromech-txzq), so appends to the open journal may be reported late. The current mitigation, reading the tail on every Status.json event, doesn't help while docked and idle, for example at an engineer or a material trader. A cheap fix is to also read the tail on the timeout: `JournalTail::read_new` opens the file and reads from the offset, which costs almost nothing when nothing has changed. This wasn't changed in astromech-txzq because another branch was changing the core watch setup. The app window doesn't use the core for journal data today (only `status`), so this affects voice, not the pages.
