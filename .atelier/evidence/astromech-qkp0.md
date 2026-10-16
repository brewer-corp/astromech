---
created_at: "2026-10-16T20:11:51.919091153+00:00"
id: "astromech-qkp0"
evidence_type: "test"
commit: "27e5c4be4296ff0a3e2d702f5cd1049271a84e59"
result: "pass"
captured_at: "2026-10-16T20:11:44.823271572+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history logbook 2>&1 | grep -E \"^test \"; cargo test -p astromech-history read_only 2>&1 | grep -E \"^test \"; cd app && pnpm -s test 2>&1 | grep -E \"Test Files|Tests \"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-fxp0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-fxp0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history logbook 2>&1 | grep -E \"^test \"; cargo test -p astromech-history read_only 2>&1 | grep -E \"^test \"; cd app && pnpm -s test 2>&1 | grep -E \"Test Files|Tests \"'"
updated_at: "2026-10-16T20:11:51.919365004+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history logbook 2>&1 | grep -E "^test "; cargo test -p astromech-history read_only 2>&1 | grep -E "^test "; cd app && pnpm -s test 2>&1 | grep -E "Test Files|Tests "'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history logbook 2>&1 | grep -E "^test "; cargo test -p astromech-history read_only 2>&1 | grep -E "^test "; cd app && pnpm -s test 2>&1 | grep -E "Test Files|Tests "'
```

Exit status: 0

## Stdout

Bytes: 371
Truncated: no

```text
test tests::logbook_search_finds_places_with_their_counts ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.01s
test tests::read_only_open_reads_what_the_writer_wrote ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.01s
 Test Files  6 passed (6)
      Tests  22 passed (22)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

