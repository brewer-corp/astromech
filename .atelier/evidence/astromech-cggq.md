---
created_at: "2026-10-23T01:23:52.123099519+00:00"
id: "astromech-cggq"
evidence_type: "test"
commit: "c0b68dfc0fab93f58e5d745cd00b97cdce96e9ba"
result: "pass"
captured_at: "2026-10-23T01:23:18.802047568+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -q -- -D warnings && cargo test --workspace -q'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-ayw1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ayw1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Rust after the plotted-resume fix: fmt, clippy -D warnings, cargo test --workspace"
updated_at: "2026-10-23T01:23:52.123401663+00:00"
---

## Summary

Rust after the plotted-resume fix: fmt, clippy -D warnings, cargo test --workspace

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -q -- -D warnings && cargo test --workspace -q'
```

Exit status: 0

## Stdout

Bytes: 15608
Truncated: yes

```text

running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
..........
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 23 tests
.......................
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 179 tests
....................................................................................... 87/179
....................................................................................... 174/179
.....
test result: ok. 179 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.79s


running 10 tests
..........
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.64s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s


running 21 tests
.....................
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.56s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.43s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.53s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s


running 7 tests
.......
test result: ok. 7
```

## Stderr

Bytes: 0
Truncated: no

```text
```

