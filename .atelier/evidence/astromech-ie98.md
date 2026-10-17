---
created_at: "2026-10-17T00:03:38.541974332+00:00"
id: "astromech-ie98"
evidence_type: "test"
commit: "e3cd7deb8351358554dcfecde039bb0fd64e9a96"
result: "pass"
captured_at: "2026-10-17T00:03:38.133762421+00:00"
command: "cargo test -p astromech-core --lib eddn_log_reads_the_outbox_without_creating_it"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6xyo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6xyo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R4: discarded rows are not Last error (fails with Discarded in the last_error filter)"
updated_at: "2026-10-17T00:03:38.542392333+00:00"
---

## Summary

R4: discarded rows are not Last error (fails with Discarded in the last_error filter)

## Command

```console
cargo test -p astromech-core --lib eddn_log_reads_the_outbox_without_creating_it
```

Exit status: 0

## Stdout

Bytes: 183
Truncated: no

```text

running 1 test
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 71 filtered out; finished in 0.21s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-094d1cbbf733166e)
```

