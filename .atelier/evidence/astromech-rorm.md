---
created_at: "2026-10-16T19:16:46.410238345+00:00"
id: "astromech-rorm"
evidence_type: "test"
commit: "4a43d0804ae0e8b29ee9beba1cf1d4ef0ba0909c"
result: "pass"
captured_at: "2026-10-16T19:16:44.270658540+00:00"
command: "cargo test -p astromech-core calibration"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ny3y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ny3y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Settings path (calibrate_* control commands) and CLI path (--calibrate-files WAVs) store the same per-device gate under the same device key; mutation-checked (keying by the raw setting fails it)"
updated_at: "2026-10-16T19:16:46.410691500+00:00"
---

## Summary

Settings path (calibrate_* control commands) and CLI path (--calibrate-files WAVs) store the same per-device gate under the same device key; mutation-checked (keying by the raw setting fails it)

## Command

```console
cargo test -p astromech-core calibration
```

Exit status: 0

## Stdout

Bytes: 585
Truncated: no

```text

running 4 tests
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test calibration::tests::steps_and_errors ... ok
test calibration::tests::level_check_verdicts ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 34 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s
```

## Stderr

Bytes: 444
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.01s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8c448b21aa4d9de9)
     Running unittests src/main.rs (target/debug/deps/astromech_core-bfdf364621cc1cce)
     Running tests/gear_down.rs (target/debug/deps/gear_down-5e63456c58497e13)
```

