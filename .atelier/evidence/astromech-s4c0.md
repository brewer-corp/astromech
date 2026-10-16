---
created_at: "2026-10-16T21:09:11.268497326+00:00"
id: "astromech-s4c0"
evidence_type: "test"
commit: "9f8be5cac2051ab63dac3aa9d33472039b01a574"
result: "pass"
captured_at: "2026-10-16T21:08:56.969055814+00:00"
command: "bash -c 'cargo test --workspace -q && cargo clippy --workspace --all-targets -q -- -D warnings && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && (cd app && pnpm test)' -q"
exit_status: "0"
agent_identity: "orchestrator (Claude)"
target:
  kind: "issue"
  id: "astromech-sd56"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-sd56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full checks at the approved branch head: workspace tests, clippy -D warnings, dependency rules, shell lock, app tests"
updated_at: "2026-10-16T21:09:11.269016826+00:00"
---

## Summary

Full checks at the approved branch head: workspace tests, clippy -D warnings, dependency rules, shell lock, app tests

## Command

```console
bash -c 'cargo test --workspace -q && cargo clippy --workspace --all-targets -q -- -D warnings && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && (cd app && pnpm test)' -q
```

Exit status: 0

## Stdout

Bytes: 6261
Truncated: yes

```text

running 57 tests
.........................................................
test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.53s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s


running 6 tests
i.....
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.70s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 12 tests
............
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 19 tests
...................
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 11 tests
...........
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 28 tests
............................
test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 24 tests
........................
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
...........
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 12 tests
............
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 11 tests
...........
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 28 tests
............................
test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok
```

## Stderr

Bytes: 0
Truncated: no

```text
```

