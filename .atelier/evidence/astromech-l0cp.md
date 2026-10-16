---
created_at: "2026-10-16T20:51:37.967717059+00:00"
id: "astromech-l0cp"
evidence_type: "test"
commit: "23f2a9c4e0905476eb240da37d83a693fc1fdb01"
result: "pass"
captured_at: "2026-10-16T20:51:37.316625424+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E \"^test |test result\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3tlm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3tlm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E \"^test |test result\"'"
updated_at: "2026-10-16T20:51:37.972171567+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E "^test |test result"'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E "^test |test result"'
```

Exit status: 0

## Stdout

Bytes: 302
Truncated: no

```text
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.12s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

