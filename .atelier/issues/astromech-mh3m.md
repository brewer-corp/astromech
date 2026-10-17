---
created_at: "2026-10-16T18:38:18.719728124+00:00"
id: "astromech-mh3m"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mh3m
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
closed_at: "2026-10-17T17:14:42.725712735+00:00"
status: "done"
title: "Core reads the journal only on file notifications"
updated_at: "2026-10-17T17:14:42.725712735+00:00"
---

## Goal

Make the core pick up journal lines promptly on Windows even when Status.json isn't changing, so voice-side game state doesn't lag.

## Done when

The core applies a line appended to the open journal within 1 s without any Status.json, Cargo.json or NavRoute.json change, and a test guards it.

## Context

`crates/astromech-core/src/pipeline.rs` reads the journal tail only when a notify event names a journal or a companion file. The 250 ms `recv_timeout` reads nothing. On Windows, ReadDirectoryChangesW reports size and last-write changes from the same lazily updated directory entry that delayed the Materials page (astromech-txzq), so appends to the open journal may be reported late. The current mitigation, reading the tail on every Status.json event, doesn't help while docked and idle, for example at an engineer or a material trader. A cheap fix is to also read the tail on the timeout: `JournalTail::read_new` opens the file and reads from the offset, which costs almost nothing when nothing has changed. This wasn't changed in astromech-txzq because another branch was changing the core watch setup. The app window doesn't use the core for journal data today (only `status`), so this affects voice, not the pages.

## Notes

- 2026-10-17 [note] Root cause: the pipeline loop only read files in response to notify events; the 250 ms recv_timeout arm did nothing, so any lost or late Windows notification (for the open journal) left the journal unread while docked and idle. Fix: Tailer::on_idle on timeout reads the tail (open + read at offset; a no-op when nothing was appended) and looks for a new journal once a second. Same class as astromech-ogq4 (work done only when told). Unverified on real Windows.

## Close Reason

Tailer::on_idle reads the journal tail on every 250 ms timeout and scans for a new journal each second; tests a_line_appended_to_the_open_journal_is_applied_when_idle and an_idle_pipeline_follows_a_new_journal_within_a_second (mutation-checked). Not verified on real Windows; review outstanding.
