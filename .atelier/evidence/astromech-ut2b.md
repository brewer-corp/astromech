---
created_at: "2026-10-16T20:01:56.998079375+00:00"
id: "astromech-ut2b"
evidence_type: "test"
commit: "06e57df93c762e3446fcf1301685994c036625c7"
result: "pass"
captured_at: "2026-10-16T20:01:55.052232431+00:00"
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
title: "astromech-core pipeline at 06e57df: startup replay queues nothing (companion file included); appended live Docked and Market are queued; a rolled-over journal is read from its Fileheader with the new header. Mutation self.live = true during replay makes it fail (3 queued vs 0)."
updated_at: "2026-10-16T20:01:56.998322419+00:00"
---

## Summary

astromech-core pipeline at 06e57df: startup replay queues nothing (companion file included); appended live Docked and Market are queued; a rolled-over journal is read from its Fileheader with the new header. Mutation self.live = true during replay makes it fail (3 queued vs 0).

## Command

```console
cargo test -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 426
Truncated: no

```text

running 4 tests
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s
```

## Stderr

Bytes: 407
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz-wire/crates/astromech-eddn)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz-wire/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.86s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3957745c62d68071)
```

