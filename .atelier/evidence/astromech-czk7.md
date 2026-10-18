---
created_at: "2026-10-18T02:44:08.033961256+00:00"
id: "astromech-czk7"
evidence_type: "test"
commit: "1aadd5c729f50fd4a872d1383a051e5e6da28fc6"
result: "pass"
captured_at: "2026-10-18T02:43:44.545974328+00:00"
command: "cargo test -p astromech-intent -p astromech-core -p astromech-store --locked -q"
exit_status: "0"
agent_identity: "claude (author agent)"
target:
  kind: "issue"
  id: "astromech-vkzj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-vkzj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "intent, core and store tests incl. strict_commands_ask_on_sound_alikes and key_presses_ask_on_sound_alikes"
updated_at: "2026-10-18T02:44:08.034233133+00:00"
---

## Summary

intent, core and store tests incl. strict_commands_ask_on_sound_alikes and key_presses_ask_on_sound_alikes

## Command

```console
cargo test -p astromech-intent -p astromech-core -p astromech-store --locked -q
```

Exit status: 0

## Stdout

Bytes: 2566
Truncated: no

```text

running 167 tests
....................................................................................... 87/167
................................................................................
test result: ok. 167 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.55s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.89s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s


running 21 tests
.....................
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.61s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.41s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.29s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
i..........
test result: ok. 10 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s


running 25 tests
.........................
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s


running 17 tests
.................
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

