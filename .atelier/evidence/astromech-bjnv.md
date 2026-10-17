---
created_at: "2026-10-17T00:49:01.081157375+00:00"
id: "astromech-bjnv"
evidence_type: "test"
commit: "e07b76d78aebadf1e94a2b4fac6fdada1e05949d"
result: "pass"
captured_at: "2026-10-17T00:48:59.243200964+00:00"
command: "cargo test -p astromech-core notifications_run"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-o7jl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-o7jl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Notifier test polls with 10 s deadline; 20/20 passes under 2x-core CPU load; fails with gap rule removed"
updated_at: "2026-10-17T00:49:01.081523669+00:00"
---

## Summary

Notifier test polls with 10 s deadline; 20/20 passes under 2x-core CPU load; fails with gap rule removed

## Command

```console
cargo test -p astromech-core notifications_run
```

Exit status: 0

## Stdout

Bytes: 877
Truncated: no

```text

running 1 test
test callouts::tests::notifications_run_in_the_background_at_most_one_per_gap ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 81 filtered out; finished in 1.52s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s
```

## Stderr

Bytes: 645
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-6ce2077a9ea40510)
     Running unittests src/main.rs (target/debug/deps/astromech_core-3b3c58feea08b5b9)
     Running tests/copy.rs (target/debug/deps/copy-c75b178883e0ab3b)
     Running tests/gear_down.rs (target/debug/deps/gear_down-7d478d23c11c17a9)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-8c024b4048eed3b2)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-c77e905fe2fbfc87)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-82dd86370bcf8d2e)
```

