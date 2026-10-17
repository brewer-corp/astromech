---
created_at: "2026-10-17T17:14:42.602898282+00:00"
id: "astromech-drqd"
evidence_type: "test"
commit: "816eb7ada726a303f9774ae63dd7215577a3171e"
result: "pass"
captured_at: "2026-10-17T17:14:36.409637527+00:00"
command: "cargo test -p astromech-core --lib pipeline"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mh3m"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mh3m"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib pipeline"
updated_at: "2026-10-17T17:14:42.603259245+00:00"
---

## Summary

cargo test -p astromech-core --lib pipeline

## Command

```console
cargo test -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 1115
Truncated: no

```text

running 12 tests
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test pipeline::tests::an_idle_pipeline_follows_a_new_journal_within_a_second ... ok
test pipeline::tests::a_line_appended_to_the_open_journal_is_applied_when_idle ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out; finished in 0.06s
```

## Stderr

Bytes: 325
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.95s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-7002ff8668dca506)
```

