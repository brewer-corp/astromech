---
schema: atelier.item
schema_version: 3
id: astromech-rggm
title: Check Materials page timing against the real game on Windows
status: ready
mission: astromech-610m
labels:
- needs-human
- size-s
- validation
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T17:17:34.564712096Z
trivial: null
caused_by: null
created_at: 2026-10-16T18:38:18.687863408Z
---

## Goal

astromech-txzq fixed a Windows-only delay that couldn't be reproduced on Linux: the Materials cache was keyed on directory-listing metadata, which NTFS can leave stale while the game holds the journal open. Confirm the fix in the real game on Windows.

## Done when

- [ ] With the app window on Materials, five material changes made in game (collect, discard, trader swap, engineer roll, synthesis) each show on the page within 2 s (evidence: a screen recording or timed notes).

## Context

`crates/astromech-materials/tests/live_journal.rs` guards this on the Windows CI job, but only with a writer handle in the same process, not with the game itself. While there, also note whether voice-side game state lags while docked and idle (see the follow-up on the core reading the journal only on file notifications).

## Notes

- 2026-10-16 [note] Also confirm the regression test crates/astromech-materials/tests/live_journal.rs would fail on Windows with the old journal_folder_sig key (review finding R2 on astromech-txzq): it only bites on Windows.
- 2026-10-17 [note] 17 Oct: also check the Engineering Builds page against the real game on Windows (astromech-ztrb changed it to read journal appends through an open handle and poll every 1 s).

## Links

Relationships of the old record that the new format has no field for.

- derived_from: astromech-txzq (Journal changes take 30 s to a minute to reach the UI)
