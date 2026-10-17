---
created_at: "2026-10-17T19:12:38.772607234+00:00"
id: "astromech-lcqa"
evidence_type: "test"
commit: "fff6ec893d3ef8c7aae0487592e0ab5136d8bfdc"
result: "pass"
captured_at: "2026-10-17T19:11:17.734988716+00:00"
command: "cargo test -p astromech-core --lib pipeline"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-7uj3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-7uj3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib pipeline"
updated_at: "2026-10-17T19:12:38.772962797+00:00"
---

## Summary

cargo test -p astromech-core --lib pipeline

## Command

```console
cargo test -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 1814
Truncated: no

```text

running 20 tests
test pipeline::tests::a_pending_event_whose_file_never_comes_is_given_up_once ... ok
test pipeline::tests::a_line_appended_to_the_open_journal_is_applied_when_idle ... ok
test pipeline::tests::a_retry_never_runs_ahead_of_the_journal_lines_of_its_batch ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_when_idle ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_on_the_next_status_read ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test pipeline::tests::backpack_json_is_read_again_after_a_backpack_change ... ok
test pipeline::tests::an_idle_pipeline_follows_a_new_journal_within_a_second ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay ... ok
test pipeline::tests::a_reopened_journal_replays_only_what_was_read_before ... ok
test pipeline::tests::a_journal_left_while_empty_is_live_when_reopened ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test pipeline::tests::a_journal_left_twice_is_split_at_the_latest_read_point ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test pipeline::tests::the_pipeline_loop_reads_the_journal_without_any_notification ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 104 filtered out; finished in 0.51s
```

## Stderr

Bytes: 1931
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-journal)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-materials)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-powerplay)
   Compiling astromech-exobiology v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-exobiology)
   Compiling astromech-exploration v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-exploration)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-engineering)
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-colonisation)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-gamestate)
   Compiling astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-history)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-galaxy)
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-announce)
   Compiling astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-station)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.47s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
```

