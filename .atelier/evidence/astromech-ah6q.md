---
created_at: "2026-10-18T01:18:25.856410964+00:00"
id: "astromech-ah6q"
evidence_type: "test"
commit: "c2a5df6d337c415501947b9c2805eb6598e32159"
result: "pass"
captured_at: "2026-10-18T01:17:29.936496866+00:00"
command: "cargo test -p astromech-intent -p astromech-core -p astromech-store --locked -q"
exit_status: "0"
agent_identity: "claude (author agent)"
target:
  kind: "issue"
  id: "astromech-1c66"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-1c66"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Re-run in a dedicated target dir (the previous receipt failed to compile against a stale astromech-intent from a shared target dir used by another worktree): intent, core and store tests incl. the 1c66 tests"
updated_at: "2026-10-18T01:18:25.856855874+00:00"
---

## Summary

Re-run in a dedicated target dir (the previous receipt failed to compile against a stale astromech-intent from a shared target dir used by another worktree): intent, core and store tests incl. the 1c66 tests

## Command

```console
cargo test -p astromech-intent -p astromech-core -p astromech-store --locked -q
```

Exit status: 0

## Stdout

Bytes: 2563
Truncated: no

```text

running 166 tests
....................................................................................... 87/166
...............................................................................
test result: ok. 166 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.46s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.72s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s


running 21 tests
.....................
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.60s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.43s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.26s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
i..........
test result: ok. 10 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s


running 23 tests
.......................
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s


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

