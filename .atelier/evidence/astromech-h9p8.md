---
created_at: "2026-10-16T18:41:13.462528892+00:00"
id: "astromech-h9p8"
evidence_type: "test"
commit: "d78c25f80d81d79d0cdd8ef7625520ef7ff92789"
result: "pass"
captured_at: "2026-10-16T18:41:11.456756863+00:00"
command: "cargo test -p astromech-core eddn_log"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-el1v"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-el1v"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Core send-log query for Diagnostics: eddn_log reads eddn.sqlite without creating it"
updated_at: "2026-10-16T18:41:13.462800838+00:00"
---

## Summary

Core send-log query for Diagnostics: eddn_log reads eddn.sqlite without creating it

## Command

```console
cargo test -p astromech-core eddn_log
```

Exit status: 0

## Stdout

Bytes: 410
Truncated: no

```text

running 1 test
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s
```

## Stderr

Bytes: 563
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-eddn)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-9ba3d252f4cc44e5)
     Running unittests src/main.rs (target/debug/deps/astromech_core-40cf7eb94cde5beb)
     Running tests/gear_down.rs (target/debug/deps/gear_down-0691f7e8d029779a)
```

