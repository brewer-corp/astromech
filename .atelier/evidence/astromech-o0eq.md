---
created_at: "2026-10-16T20:17:03.438229756+00:00"
id: "astromech-o0eq"
evidence_type: "test"
commit: "e1a22ac97f4f739597c62fc8b80ea81720ad5092"
result: "pass"
captured_at: "2026-10-16T20:16:56.085159421+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history session_summary 2>&1 | grep -E \"^test \"; cargo test -p astromech-history visits_ledger 2>&1 | grep -E \"^test \"; cd app && pnpm -s test 2>&1 | grep -E \"Tests \"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-bg0x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-bg0x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history session_summary 2>&1 | grep -E \"^test \"; cargo test -p astromech-history visits_ledger 2>&1 | grep -E \"^test \"; cd app && pnpm -s test 2>&1 | grep -E \"Tests \"'"
updated_at: "2026-10-16T20:17:03.438642206+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history session_summary 2>&1 | grep -E "^test "; cargo test -p astromech-history visits_ledger 2>&1 | grep -E "^test "; cd app && pnpm -s test 2>&1 | grep -E "Tests "'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-history session_summary 2>&1 | grep -E "^test "; cargo test -p astromech-history visits_ledger 2>&1 | grep -E "^test "; cd app && pnpm -s test 2>&1 | grep -E "Tests "'
```

Exit status: 0

## Stdout

Bytes: 293
Truncated: no

```text
test tests::session_summary_totals ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.01s
test tests::visits_ledger ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.01s
      Tests  23 passed (23)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

