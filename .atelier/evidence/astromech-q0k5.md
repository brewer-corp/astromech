---
created_at: "2026-10-17T19:37:35.823812167+00:00"
id: "astromech-q0k5"
evidence_type: "test"
commit: "6448c00a9646d5127fab0ff799a36b9f9964d3dc"
result: "pass"
captured_at: "2026-10-17T19:37:35.419256267+00:00"
command: "cargo test -p astromech-eddn --test documented_limits"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-koa6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-koa6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-eddn --test documented_limits"
updated_at: "2026-10-17T19:37:35.827471151+00:00"
---

## Summary

cargo test -p astromech-eddn --test documented_limits

## Command

```console
cargo test -p astromech-eddn --test documented_limits
```

Exit status: 0

## Stdout

Bytes: 176
Truncated: no

```text

running 1 test
test the_page_and_the_guide_state_the_senders_age_limits ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 168
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running tests/documented_limits.rs (target/debug/deps/documented_limits-3fd25514958bbffa)
```

