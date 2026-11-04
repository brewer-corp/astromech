---
created_at: "2026-10-22T21:06:57.006610647+00:00"
id: "astromech-zftk"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-zftk
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
closed_at: "2026-11-04T21:43:11.802753628+00:00"
status: "done"
title: "Journal catch-up: lines the game writes during the startup replay count as history"
updated_at: "2026-11-04T22:57:03.027515588+00:00"
---

## Goal

Lines the game writes to its current journal while astromech replays at startup are treated as live (sent and spoken), so no live line is missed in that window.

## Done when

A pipeline test that appends to the game's current journal, or creates a new journal, while catch_up is replaying sees those lines handled as live, and history written before startup is still not sent.

## Context

Review finding R3 (astromech-dsrp) on feature/astromech-g3qt: catch_up (crates/astromech-core/src/pipeline.rs) measures startup lengths after replaying the followed journal. Measuring first narrows the window but changes EDDN behaviour during catch-up, so it needs its own look.

## Notes

- 2026-11-04 [decision] Measure every journal's startup length before the catch-up reads anything (the followed one through an open handle, like the latest by name), and during the catch-up replay the followed journal only up to that length; what lies past it is left for the first read after caught_up, so it goes through the normal live path (EDDN, EDSM, callouts, companion files).
  Reason: Measuring first closes the window for the game's current journal and for a journal it starts during the replay. Reading the followed journal's remainder inside the catch-up would send it to EDDN (which uses self.live) while callouts are still off (caught_up false), so lines would be sent but not spoken; deferring it to the next read (at most 250 ms) gives one notion of live and needs no new state. The followed journal is measured through a handle because a lagging directory listing would make history live.
- 2026-11-04 [note] Resolution of review findings astromech-6rig and astromech-p58u (36f1203f): the followed journal's startup length is taken only through an open handle; if that fails it is UNMEASURED (u64::MAX), so everything the catch-up reads of it is history (fail closed: lines the game writes during that catch-up are lost to history rather than history going live). open() returns without changing tail or opened when the file can't be opened, so a followed journal unreadable during the catch-up leaves tail None and is opened by follow_newest later with its startup length (or UNMEASURED) as the split. Tests: a_followed_journal_that_cannot_be_measured_is_replayed_in_full, a_followed_journal_unreadable_through_the_catch_up_is_history_once_read, a_followed_journal_unreadable_for_the_replay_is_split_at_its_startup_length (the latter two and the first drive catch_up's steps directly, as no hook sits between measuring and opening). Open: with tail None after such a catch-up, follow_newest opens the newest by mtime, which may be a touched older journal until the game's becomes readable.

## Close Reason

Implementer report. Changed (crates/astromech-core/src/pipeline.rs): catch_up measures every journal's startup length before reading anything (note_startup_lengths now takes the followed journal and measures it through an open handle, like the latest by name, and overwrites rather than or_insert); open() during the catch-up replays a journal with a known length only up to it and leaves the rest for the first read after caught_up, so it takes the normal live path with callouts on. Tests: a_line_the_game_writes_to_its_journal_during_the_catch_up_is_live and a_journal_the_game_starts_during_the_catch_up_is_live (fail before the fix), the_followed_journal_past_its_startup_length_is_live_after_the_catch_up (a guard: the window inside the followed journal's own read can't be hooked from a test, so it passes before the fix too); writes happen from inside catch_up via the log hook. Docs: pipeline module doc, ARCHITECTURE.md 5.2 startup bullet. Deviations: four reopen tests (conflict copy, 7uj3, left twice, left while empty) opened a journal with no catch-up (caught_up false); they now start with catch_up, since the remainder of a known journal is now held back while caught_up is false. Surprises: none in behaviour; the reviewer's EDDN concern is avoided by not reading past the startup length during the catch-up at all. Open items: (1) lines the game writes during the catch-up reach EDDN and callouts up to 250 ms (one idle read) after it ends, and a journal started meanwhile is found by the once-a-second scan or the next notification; (2) if the followed journal can't be opened when measured, the listing length is used, which on Windows can lag, so lines just before startup could be live; (3) independent review outstanding.
