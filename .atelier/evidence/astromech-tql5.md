---
created_at: "2026-10-17T23:41:00.411036914+00:00"
id: "astromech-tql5"
evidence_type: "test"
commit: "096a53d1f8342a7a2ed70d1d82455dbf4bde30bb"
result: "pass"
captured_at: "2026-10-17T23:41:00.108917791+00:00"
command: "cargo test -p astromech-core --lib controls"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-e0zz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e0zz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib controls"
updated_at: "2026-10-17T23:41:00.411284115+00:00"
---

## Summary

cargo test -p astromech-core --lib controls

## Command

```console
cargo test -p astromech-core --lib controls
```

Exit status: 0

## Stdout

Bytes: 708
Truncated: no

```text

running 9 tests
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::paste_button_fires_once_per_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::the_wait_is_the_idle_tick_except_to_end_a_debounce_window ... ok
test controls::tests::release_is_reported_when_the_debounce_window_ends ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 154 filtered out; finished in 0.00s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-6845259f087e785f)
```

