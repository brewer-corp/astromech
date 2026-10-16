---
created_at: "2026-10-16T18:38:18.687863408+00:00"
id: "astromech-rggm"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
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
    id: "astromech-txzq"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check Materials page timing against the real game on Windows"
updated_at: "2026-10-16T18:44:37.944173658+00:00"
---

## Goal

astromech-txzq fixed a Windows-only delay that couldn't be reproduced on Linux: the Materials cache was keyed on directory-listing metadata, which NTFS can leave stale while the game holds the journal open. Confirm the fix in the real game on Windows.

## Done when

With the app window on Materials, five material changes made in game (collect, discard, trader swap, engineer roll, synthesis) each show on the page within 2 s (evidence: a screen recording or timed notes).

## Context

`crates/astromech-materials/tests/live_journal.rs` guards this on the Windows CI job, but only with a writer handle in the same process, not with the game itself. While there, also note whether voice-side game state lags while docked and idle (see the follow-up on the core reading the journal only on file notifications).

## Notes

- 2026-10-16 [note] Also confirm the regression test crates/astromech-materials/tests/live_journal.rs would fail on Windows with the old journal_folder_sig key (review finding R2 on astromech-txzq): it only bites on Windows.
