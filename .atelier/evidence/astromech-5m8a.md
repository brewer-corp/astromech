---
created_at: "2026-10-17T00:27:33.086002490+00:00"
id: "astromech-5m8a"
evidence_type: "test"
commit: "467ae488f07bd2be3d1861f435d8e3cf0e166a86"
result: "pass"
captured_at: "2026-10-17T00:27:27.758178990+00:00"
command: "cargo test -p astromech-core --lib a_system_spansh_doesnt_know"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-970a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-970a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R7: voice lookup searches from StarPos (works for a system Spansh doesn't know); without a position Spansh's 400 is said plainly; failed before: 'Spansh turned the request down: Invalid request.'"
updated_at: "2026-10-17T00:27:33.086436300+00:00"
---

## Summary

R7: voice lookup searches from StarPos (works for a system Spansh doesn't know); without a position Spansh's 400 is said plainly; failed before: 'Spansh turned the request down: Invalid request.'

## Command

```console
cargo test -p astromech-core --lib a_system_spansh_doesnt_know
```

Exit status: 0

## Stdout

Bytes: 215
Truncated: no

```text

running 1 test
test lookups::tests::a_system_spansh_doesnt_know_is_searched_from_its_position_or_said_plainly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 71 filtered out; finished in 0.00s
```

## Stderr

Bytes: 219
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.20s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-094d1cbbf733166e)
```

