---
created_at: "2026-10-16T20:01:14.086238737+00:00"
id: "astromech-mbt3"
evidence_type: "test"
commit: "8069aac300c0a77cfa5f3a69a4f0b58c02e916a4"
result: "pass"
captured_at: "2026-10-16T20:01:12.650236778+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history -p astromech-journal 2>&1 | grep -E \"older_journal|ship_rows_ignore|docked_keeps|killed_anywhere|v1_database|test result\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-u3f4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-u3f4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history -p astromech-journal 2>&1 | grep -E \"older_journal|ship_rows_ignore|docked_keeps|killed_anywhere|v1_database|test result\"'"
updated_at: "2026-10-16T20:01:14.086508019+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history -p astromech-journal 2>&1 | grep -E "older_journal|ship_rows_ignore|docked_keeps|killed_anywhere|v1_database|test result"'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history -p astromech-journal 2>&1 | grep -E "older_journal|ship_rows_ignore|docked_keeps|killed_anywhere|v1_database|test result"'
```

Exit status: 0

## Stdout

Bytes: 756
Truncated: no

```text
test tests::ship_rows_ignore_older_events ... ok
test tests::older_journal_imported_late_gives_the_same_ledgers ... ok
test tests::v1_database_is_migrated ... ok
test tests::import_killed_anywhere_then_resumed_equals_clean_import ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.44s
test tests::docked_keeps_system_address ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

