---
created_at: "2026-11-04T22:37:46.901442567+00:00"
id: "astromech-rjhy"
issue_type: "bug"
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
  - kind: "review"
    id: "astromech-0576"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Journal startup lengths: measure recent journals through a handle"
updated_at: "2026-11-04T23:34:17.339716666+00:00"
---

## Goal

Lines the game wrote to its current journal before astromech started are never live, even when that journal is neither the latest by name nor the one the catch-up follows.

## Done when

Every journal whose name stamp falls within the last day is measured through an open handle at startup (not the directory listing, which lags on Windows for a file the game holds open), with a pipeline test where a later-stamped foreign journal is in the folder and the game's journal is measured by handle.

## Context

Review finding R3 (astromech-0576) on bug/astromech-zftk, pre-existing: crates/astromech-core/src/pipeline.rs:705-712.

## Notes

- 2026-11-04 [decision] At startup, measure through an open handle every journal whose name stamp is within 24 h of the local wall clock (or later), besides the followed and latest-by-name ones; same listing fallback as ek06 when the handle fails (followed keeps UNMEASURED). Stats.lengths_by_handle counts them.
  Reason: The game holds open only the journal it is writing, which was started at most a session ago; a day covers a long session and its parts while keeping startup to one listing plus a handful of opens on a folder of years of journals. Names carry local time, so the cutoff is the local clock read as UTC like journal_name_stamp. A session longer than 24 h, or a journal named by a clock more than a day behind, is still measured by the listing.
