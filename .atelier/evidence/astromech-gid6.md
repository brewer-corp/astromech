---
created_at: "2026-10-17T19:28:00.659426228+00:00"
id: "astromech-gid6"
evidence_type: "test"
commit: "2ce485d2a5df9b7f6307a1af1e06169d9d7bc0d4"
result: "pass"
captured_at: "2026-10-17T19:24:04.137491810+00:00"
command: "bash -c 'set -eo pipefail; export CARGO_BUILD_JOBS=3; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | tail -n 200; python3 scripts/check-deps.py; python3 scripts/check-guide.py'"
exit_status: "0"
agent_identity: "claude-author (Opus)"
target:
  kind: "issue"
  id: "astromech-8sx3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-8sx3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Gates at HEAD: fmt --check, clippy -D warnings, cargo test --workspace (incl. crates/astromech-core/tests/known_bodies.rs: recorded EDSM answer -> '2 of 9 bodies known.' on a live jump, silent catch-up with no request, offline silent + logged + jump under 200 ms, slow 600 ms answer no stall and dropped after the minute; copy.rs; announce replay catch-up with 19 rules; biology provider tests on the shared background helper), check-deps, check-guide."
updated_at: "2026-10-17T19:28:00.659974029+00:00"
---

## Summary

Gates at HEAD: fmt --check, clippy -D warnings, cargo test --workspace (incl. crates/astromech-core/tests/known_bodies.rs: recorded EDSM answer -> '2 of 9 bodies known.' on a live jump, silent catch-up with no request, offline silent + logged + jump under 200 ms, slow 600 ms answer no stall and dropped after the minute; copy.rs; announce replay catch-up with 19 rules; biology provider tests on the shared background helper), check-deps, check-guide.

## Command

```console
bash -c 'set -eo pipefail; export CARGO_BUILD_JOBS=3; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | tail -n 200; python3 scripts/check-deps.py; python3 scripts/check-guide.py'
```

Exit status: 0

## Stdout

Bytes: 4727
Truncated: yes

```text

running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
..........
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s


running 32 tests
................................
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 30 tests
..............................
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 te
```

## Stderr

Bytes: 60
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
```

