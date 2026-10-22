---
created_at: "2026-10-22T21:03:08.793681906+00:00"
id: "astromech-ar8j"
evidence_type: "test"
commit: "66d97a389deab854a52b5273b49bbd3ac365ca6d"
result: "pass"
captured_at: "2026-10-22T21:03:08.492571167+00:00"
command: "cargo test -p astromech-eddn --test documented_limits"
exit_status: "0"
agent_identity: "claude-implementer-ux0h"
target:
  kind: "issue"
  id: "astromech-otsm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-otsm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "documented_limits asserts whole sentences for both limits on the page and in the guide, 8 and 2,000 built from MAX_ATTEMPTS and MAX_QUEUED; each mutated sentence fails it"
updated_at: "2026-10-22T21:03:08.793989520+00:00"
---

## Summary

documented_limits asserts whole sentences for both limits on the page and in the guide, 8 and 2,000 built from MAX_ATTEMPTS and MAX_QUEUED; each mutated sentence fails it

## Command

```console
cargo test -p astromech-eddn --test documented_limits
```

Exit status: 0

## Stdout

Bytes: 219
Truncated: no

```text

running 2 tests
test thousands_are_separated_by_commas ... ok
test the_page_and_the_guide_state_the_senders_limits ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 168
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running tests/documented_limits.rs (target/debug/deps/documented_limits-120709a72c92d6a3)
```

