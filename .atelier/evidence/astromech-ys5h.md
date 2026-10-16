---
created_at: "2026-10-16T18:39:44.111686969+00:00"
id: "astromech-ys5h"
evidence_type: "test"
commit: "fef8777e3cb7429da691dd961c6df40791516005"
result: "pass"
captured_at: "2026-10-16T18:39:43.802670437+00:00"
command: "cargo test -p astromech-engineering fixture_fleet"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mycc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mycc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-engineering fixture_fleet"
updated_at: "2026-10-16T18:39:44.112071626+00:00"
---

## Summary

cargo test -p astromech-engineering fixture_fleet

## Command

```console
cargo test -p astromech-engineering fixture_fleet
```

Exit status: 0

## Stdout

Bytes: 181
Truncated: no

```text

running 1 test
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.04s
```

## Stderr

Bytes: 166
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ff71dd009829e65d)
```

