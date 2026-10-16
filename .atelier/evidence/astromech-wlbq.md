---
created_at: "2026-10-16T19:21:58.792960607+00:00"
id: "astromech-wlbq"
evidence_type: "test"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:21:58.078977862+00:00"
command: "cargo test -p astromech-ship --lib"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-ship --lib"
updated_at: "2026-10-16T19:21:58.793223208+00:00"
---

## Summary

cargo test -p astromech-ship --lib

## Command

```console
cargo test -p astromech-ship --lib
```

Exit status: 0

## Stdout

Bytes: 461
Truncated: no

```text

running 6 tests
test tests::capacitor_recharge_follows_the_pip_curve ... ok
test tests::unknown_ship_is_an_error ... ok
test tests::unpowered_drive_has_no_range ... ok
test module::tests::unknown_grade_is_reported ... ok
test tests::unknown_modules_and_blueprints_are_reported_not_counted ... ok
test module::tests::grade_5_at_roll_0_is_the_grade_4_maximum ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 278
Truncated: no

```text
   Compiling astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-ship)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-8eaa524fdea4b627)
```

