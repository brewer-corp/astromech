---
created_at: "2026-10-16T20:38:24.348175437+00:00"
id: "astromech-5t34"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-5t34
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-ipb5"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Start or re-point the history follower when the journal folder changes"
updated_at: "2026-10-16T23:55:32.126559734+00:00"
---

## Goal

The core starts the history follower once at startup from a config snapshot. On a first run with no journal folder found, onboarding (set_folders) sets one, but history never starts, so Logbook and Session say "no journal history yet" until restart; later journal_dir changes are ignored too.

## Done when

After a successful set_folders the follower starts (or re-points) without a restart; a core test shows a folder set after startup gets imported.

## Context

`crates/astromech-core/src/app.rs` (startup history start), `history.rs` (OnceLock follower). Found in review of the astromech-ipb5 main merge.
