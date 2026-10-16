---
created_at: "2026-10-16T18:12:15.204408970+00:00"
id: "astromech-txzq"
issue_type: "bug"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-txzq
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Journal changes take 30 s to a minute to reach the UI"
updated_at: "2026-10-16T18:27:57.341259644+00:00"
---

## Goal

The app feels laggy. Changes to engineering materials often take 30 s to a minute to show in the UI after the game writes them, and journal updates in general are not always picked up quickly. Find where the time goes and get journal changes to the UI within about 2 s.

## Done when

A timed check (appending fixture events to a live journal file) shows each change on the Materials page within 2 s, and a regression test or diagnostic timer guards it. Record where the delay came from as a note.

## Context

The UI polls every 2 s (Materials) and every 5 s (Builds). The core watches the journal in `crates/astromech-core/src/app.rs` (around lines 540 to 580). Some material changes may only show up as a full `Materials` event rather than the incremental events. Diagnose before changing anything.
