---
created_at: "2026-10-17T17:12:27.159074431+00:00"
id: "astromech-hxjx"
evidence_type: "test"
commit: "8bab27e1e957dae04dff5601c22c273b4c0e4935"
result: "pass"
captured_at: "2026-10-17T17:12:26.490541229+00:00"
command: "cargo test -p astromech-journal --test live_watch"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ztrb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ztrb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-journal --test live_watch"
updated_at: "2026-10-17T17:12:27.159327743+00:00"
---

## Summary

cargo test -p astromech-journal --test live_watch

## Command

```console
cargo test -p astromech-journal --test live_watch
```

Exit status: 0

## Stdout

Bytes: 331
Truncated: no

```text

running 4 tests
test a_cache_rebuilds_for_modelled_events_only ... ok
test a_new_journal_file_counts ... ok
test a_half_written_line_counts_once_it_is_finished ... ok
test typed_appends_to_an_open_journal_change_the_generation ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 266
Truncated: no

```text
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-journal)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running tests/live_watch.rs (target/debug/deps/live_watch-9983733a96279bbd)
```

