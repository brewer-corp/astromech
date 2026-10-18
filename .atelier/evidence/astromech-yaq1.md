---
created_at: "2026-10-17T23:59:22.478131398+00:00"
id: "astromech-yaq1"
evidence_type: "test"
commit: "c0cdbf393ddcb669b5859781cefa3f9b40adfc2a"
result: "pass"
captured_at: "2026-10-17T23:59:21.464624205+00:00"
command: "cargo test -p astromech-intent --locked -q"
exit_status: "0"
agent_identity: "claude (author agent)"
target:
  kind: "issue"
  id: "astromech-y296"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-y296"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-intent unit tests incl. all_stop_mishearings_ask_and_never_execute (asks, score <= cap, everyday negatives never execute)"
updated_at: "2026-10-17T23:59:22.478516426+00:00"
---

## Summary

astromech-intent unit tests incl. all_stop_mishearings_ask_and_never_execute (asks, score <= cap, everyday negatives never execute)

## Command

```console
cargo test -p astromech-intent --locked -q
```

Exit status: 0

## Stdout

Bytes: 246
Truncated: no

```text

running 18 tests
..................
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

