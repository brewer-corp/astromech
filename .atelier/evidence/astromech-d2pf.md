---
created_at: "2026-10-17T18:09:53.346688294+00:00"
id: "astromech-d2pf"
evidence_type: "test"
commit: "645347c931e2d700122ea4823434d67709b87f1b"
result: "pass"
captured_at: "2026-10-17T18:09:51.074490940+00:00"
command: "cargo test -p astromech-core --lib pipeline"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ogq4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ogq4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib pipeline"
updated_at: "2026-10-17T18:09:53.346968096+00:00"
---

## Summary

cargo test -p astromech-core --lib pipeline

## Command

```console
cargo test -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 1624
Truncated: no

```text

running 18 tests
test pipeline::tests::a_pending_event_whose_file_never_comes_is_given_up_once ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_when_idle ... ok
test pipeline::tests::a_line_appended_to_the_open_journal_is_applied_when_idle ... ok
test pipeline::tests::backpack_json_is_read_again_after_a_backpack_change ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_on_the_next_status_read ... ok
test pipeline::tests::an_idle_pipeline_follows_a_new_journal_within_a_second ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::a_retry_never_runs_ahead_of_the_journal_lines_of_its_batch ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test pipeline::tests::the_pipeline_loop_reads_the_journal_without_any_notification ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 104 filtered out; finished in 0.51s
```

## Stderr

Bytes: 593
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.60s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
```

