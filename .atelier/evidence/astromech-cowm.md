---
created_at: "2026-10-16T19:56:10.529810835+00:00"
id: "astromech-cowm"
evidence_type: "test"
commit: "1f5596b07e9405df7407d12cf0d13e230f7addbb"
result: "pass"
captured_at: "2026-10-16T19:56:09.466734902+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E \"^test |test result\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-i4gx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-i4gx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E \"^test |test result\"'"
updated_at: "2026-10-16T19:56:10.530039311+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E "^test |test result"'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E "^test |test result"'
```

Exit status: 0

## Stdout

Bytes: 681
Truncated: no

```text
test commander::tests::keys ... ok
test tests::unfinished_line_waits_for_its_newline ... ok
test tests::commanders_and_legacy_stay_apart ... ok
test tests::ships_ledger ... ok
test tests::scans_ledger ... ok
test tests::earnings_ledger ... ok
test tests::visits_ledger ... ok
test tests::imports_every_line_once ... ok
test tests::rebuilt_ledgers_equal_imported_ones ... ok
test tests::v1_database_is_migrated ... ok
test tests::import_killed_anywhere_then_resumed_equals_clean_import ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.40s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

