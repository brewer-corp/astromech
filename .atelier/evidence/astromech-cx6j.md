---
created_at: "2026-10-16T19:45:03.623067610+00:00"
id: "astromech-cx6j"
evidence_type: "test"
commit: "95d54565100f3d4a6cc438b8c5d93ba7ca70427e"
result: "pass"
captured_at: "2026-10-16T19:45:02.543924785+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E \"^test |test result\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-aonb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-aonb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E \"^test |test result\"'"
updated_at: "2026-10-16T19:45:03.623352202+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E "^test |test result"'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history 2>&1 | grep -E "^test |test result"'
```

Exit status: 0

## Stdout

Bytes: 550
Truncated: no

```text
test tests::unfinished_line_waits_for_its_newline ... ok
test tests::scans_ledger ... ok
test tests::earnings_ledger ... ok
test tests::imports_every_line_once ... ok
test tests::visits_ledger ... ok
test tests::ships_ledger ... ok
test tests::rebuilt_ledgers_equal_imported_ones ... ok
test tests::import_killed_anywhere_then_resumed_equals_clean_import ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

