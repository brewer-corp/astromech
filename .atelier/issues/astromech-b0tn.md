---
created_at: "2026-11-04T23:55:31.999191885+00:00"
id: "astromech-b0tn"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-b0tn
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
status: "in_progress"
title: "Journal following: a just-left journal touched by a sync tool"
updated_at: "2026-11-05T00:19:29.429119686+00:00"
---

## Goal

A journal the game has just left can't take the session back, and retire the game's real session, when a tool touches it within two minutes of its last line.

## Done when

In follow_newest, a journal wins by being written only if its last line is stamped after the followed journal's start; a pipeline test touches the just-left journal within 120 s after a rollover and the game's new journal keeps the session and stays Game (R1, astromech-y9qx).

## Context

Deferred review finding R1 (astromech-y9qx) on task/astromech-t34c: crates/astromech-core/src/pipeline.rs:578-614, 765-770. Nothing is wrongly sent; on Windows the game's lines can go unread for 30-60 s and some live lines count as history.

## Notes

- 2026-11-04 [note] Implementer report (bf39b03b): follow_newest's being_written branch now also requires Tailer::written_since_start(n, followed): n's last line stamped strictly after followed's start_time, or followed has no believable start time (None keeps the clock-ahead give-way behaviour). It is checked before being_written so note_writing's retire side effect doesn't run for a just-left journal. line_is_fresh split into a line_time helper. Proof: new test pipeline::tests::a_just_left_journal_touched_within_two_minutes_keeps_the_new_one_followed (old journal last line 10 s ago, rollover to new, tool touches old; asserts new stays followed, classify(new)==Game, and the next line is sent) failed before the fix (followed switched to old) and passes after. cargo fmt --check clean; cargo clippy -p astromech-core --all-targets -D warnings clean; cargo test -p astromech-core all pass (204 lib). Review outstanding.
