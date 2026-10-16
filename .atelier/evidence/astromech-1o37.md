---
created_at: "2026-10-16T20:05:06.401788862+00:00"
id: "astromech-1o37"
evidence_type: "test"
commit: "bcbf16aafb9671157dcc31d11ba88a934b833211"
result: "pass"
captured_at: "2026-10-16T20:05:01.702709872+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E \"^test |test result\"; cargo test -p astromech-history import_reports_progress 2>&1 | grep -E \"^test \"'"
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
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E \"^test |test result\"; cargo test -p astromech-history import_reports_progress 2>&1 | grep -E \"^test \"'"
updated_at: "2026-10-16T20:05:06.402184911+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E "^test |test result"; cargo test -p astromech-history import_reports_progress 2>&1 | grep -E "^test "'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-core --lib history 2>&1 | grep -E "^test |test result"; cargo test -p astromech-history import_reports_progress 2>&1 | grep -E "^test "'
```

Exit status: 0

## Stdout

Bytes: 374
Truncated: no

```text
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.09s
test tests::import_reports_progress ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.03s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

