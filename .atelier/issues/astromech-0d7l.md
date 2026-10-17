---
created_at: "2026-10-17T19:04:52.678183574+00:00"
id: "astromech-0d7l"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-0d7l
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Flaky: gear_down a_replayed_older_journal_leaves_the_route_and_clipboard_alone"
updated_at: "2026-10-17T20:05:18.845510745+00:00"
---

## Goal

`crates/astromech-core/tests/gear_down.rs::a_replayed_older_journal_leaves_the_route_and_clipboard_alone` passes reliably, or the race it exposes (a replayed older journal writing the clipboard) is found and fixed.

## Done when

The cause is known (test timing or a real replay-treated-as-live race in the pipeline), fixed, and the test passes 50 runs in a row under load (e.g. alongside `cargo test --workspace`).

## Context

Seen once on 17 Oct 2026 during `cargo test --workspace` on branch task/astromech-lo81 with other cargo builds running on the host: panicked at gear_down.rs:556, "no clipboard write from a replay" (the clipboard had more than the one copy from `Desk::set`). It passed 6 runs in a row on its own. The lo81 change only affects rules that are turned off; this rig has route_follow on, so it is unlikely to be the cause. If it is a real race, an older journal re-read could move the route and overwrite the clipboard in game.

## Notes

- 2026-10-17 [note] Diagnosis (production race, not test-only): Tailer::open set self.live=true after the replay part, and read_tail applied later lines with it. A journal earlier by name than the current one (a backup/sync tool) that is followed while empty or partly written (its creation makes it newest) had everything written after the open treated as live: route moved, clipboard written, callouts, EDDN/EDSM sends. The test's append() creates the file then writes it; the pipeline wins that gap under load. Repro: 5+ ms injected sleep between create and write gave 60/60 failures with the CI signature, against 0/30 without it and 0/40 under 24 busy loops. Same hole for the 7uj3 split on an older journal followed, left and written again. Fix: in open(), a journal is live only if it is the folder's latest by name (journal_logs().last()); that also holds for later tail reads. Tests: pipeline::tests::an_older_journal_written_after_we_follow_it_is_still_a_replay (fails before the fix with 2 EDDN sends); the gear_down test now forces the interleaving (creates the older journal empty, waits until it is followed, then writes), 5/5 fail before the fix.
