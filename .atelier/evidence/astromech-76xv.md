---
created_at: "2026-10-16T18:41:05.819082936+00:00"
id: "astromech-76xv"
evidence_type: "test"
commit: "0012a2013a9f6cba463b5c9369ade0fd39297814"
result: "pass"
captured_at: "2026-10-16T18:41:05.304261757+00:00"
command: "cargo test -p astromech-materials --test live_journal"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-txzq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-txzq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Regression guard: appends to an open journal change journal_version at once, and every material event type reaches the inventory well within the 1 s recompute budget (Linux run; the test also runs on the Windows CI job)"
updated_at: "2026-10-16T18:41:05.819273622+00:00"
---

## Summary

Regression guard: appends to an open journal change journal_version at once, and every material event type reaches the inventory well within the 1 s recompute budget (Linux run; the test also runs on the Windows CI job)

## Command

```console
cargo test -p astromech-materials --test live_journal
```

Exit status: 0

## Stdout

Bytes: 221
Truncated: no

```text

running 2 tests
test version_follows_a_new_journal_file ... ok
test appends_to_an_open_journal_show_up_within_budget ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Stderr

Bytes: 283
Truncated: no

```text
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-txzq/crates/astromech-materials)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running tests/live_journal.rs (target/debug/deps/live_journal-1d14e180d86fdb81)
```

