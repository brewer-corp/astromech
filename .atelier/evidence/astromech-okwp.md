---
created_at: "2026-10-16T20:13:27.053787524+00:00"
id: "astromech-okwp"
evidence_type: "test"
commit: "e414024f96fb7d24f4a0c5532622aa386b523ebb"
result: "pass"
captured_at: "2026-10-16T20:13:25.842031034+00:00"
command: "cargo test -p astromech-core --lib pipeline"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-xvjs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xvjs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fixes at e414024, astromech-core: an_older_journal_touched_later_is_read_as_a_replay (R1; fails, 2 queued, if every newest journal is opened live), eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it (now with fresh timestamps)"
updated_at: "2026-10-16T20:13:27.053992086+00:00"
---

## Summary

Review fixes at e414024, astromech-core: an_older_journal_touched_later_is_read_as_a_replay (R1; fails, 2 queued, if every newest journal is opened live), eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it (now with fresh timestamps)

## Command

```console
cargo test -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 506
Truncated: no

```text

running 5 tests
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s
```

## Stderr

Bytes: 397
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-eddn)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.13s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3957745c62d68071)
```

