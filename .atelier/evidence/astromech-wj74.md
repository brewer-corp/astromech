---
created_at: "2026-10-16T22:50:58.751840734+00:00"
id: "astromech-wj74"
evidence_type: "test"
commit: "45d71dd63bd3e192b31aa67f7fa896cf69cef8c4"
result: "pass"
captured_at: "2026-10-16T22:50:27.732512151+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3 && cargo fmt --all --check && cargo clippy -q --workspace --all-targets -- -D warnings && cargo test -q --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile >/dev/null && pnpm -s lint && pnpm -s check && pnpm -s build >/dev/null && pnpm -s test'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-sfys"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-sfys"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full gates at the second merge of main (trade near me, Modifiers ship stats, gear_down flake fix) plus the trade_voice Runner fix (45d71dd): fmt, clippy -D warnings, workspace tests, check-deps, app/src-tauri cargo metadata --locked --offline, app install/lint/check/build/test. Supersedes the failing receipt astromech-f3sq at 7d90c0f (trade_voice.rs lacked the Runner galaxy field). app/src-tauri main.rs is checked by reading only (Windows-only build)."
updated_at: "2026-10-16T22:50:58.755665778+00:00"
---

## Summary

Full gates at the second merge of main (trade near me, Modifiers ship stats, gear_down flake fix) plus the trade_voice Runner fix (45d71dd): fmt, clippy -D warnings, workspace tests, check-deps, app/src-tauri cargo metadata --locked --offline, app install/lint/check/build/test. Supersedes the failing receipt astromech-f3sq at 7d90c0f (trade_voice.rs lacked the Runner galaxy field). app/src-tauri main.rs is checked by reading only (Windows-only build).

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3 && cargo fmt --all --check && cargo clippy -q --workspace --all-targets -- -D warnings && cargo test -q --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile >/dev/null && pnpm -s lint && pnpm -s check && pnpm -s build >/dev/null && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 8053
Truncated: yes

```text

running 64 tests
................................................................
test result: ok. 64 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.54s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s


running 6 tests
i.....
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.72s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 12 tests
............
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 105 tests
....................................................................................... 87/105
..................
test result: ok. 105 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.92s


running 24 tests
........................
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 17 tests
.................
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 18 tests
..................
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 29 tests
.............................
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 24 tests
........................
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
...........
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
.....
```

## Stderr

Bytes: 0
Truncated: no

```text
```

