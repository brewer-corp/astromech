---
created_at: "2026-10-17T19:04:52.678183574+00:00"
id: "astromech-0d7l"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Flaky: gear_down a_replayed_older_journal_leaves_the_route_and_clipboard_alone"
updated_at: "2026-10-17T19:04:52.678183574+00:00"
---

## Goal

`crates/astromech-core/tests/gear_down.rs::a_replayed_older_journal_leaves_the_route_and_clipboard_alone` passes reliably, or the race it exposes (a replayed older journal writing the clipboard) is found and fixed.

## Done when

The cause is known (test timing or a real replay-treated-as-live race in the pipeline), fixed, and the test passes 50 runs in a row under load (e.g. alongside `cargo test --workspace`).

## Context

Seen once on 17 Oct 2026 during `cargo test --workspace` on branch task/astromech-lo81 with other cargo builds running on the host: panicked at gear_down.rs:556, "no clipboard write from a replay" (the clipboard had more than the one copy from `Desk::set`). It passed 6 runs in a row on its own. The lo81 change only affects rules that are turned off; this rig has route_follow on, so it is unlikely to be the cause. If it is a real race, an older journal re-read could move the route and overwrite the clipboard in game.
