---
created_at: "2026-10-17T19:11:35.848432623+00:00"
id: "astromech-rhfc"
evidence_type: "test"
commit: "01f619c0e55415cef47302afe1637b0c97aa2313"
result: "pass"
captured_at: "2026-10-17T19:09:10.192821621+00:00"
command: "bash -c 'set -eo pipefail; export CARGO_BUILD_JOBS=3; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | tail -n 300; python3 scripts/check-deps.py; python3 scripts/check-guide.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo metadata-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'"
exit_status: "0"
agent_identity: "claude-author (Opus)"
target:
  kind: "issue"
  id: "astromech-94j4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-4fe3"
    role: "validates"
  - kind: "issue"
    id: "astromech-94j4"
    role: "validates"
  - kind: "issue"
    id: "astromech-dqmv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "All gates at HEAD: cargo fmt --check, clippy -D warnings, cargo test --workspace (incl. recorded Spansh tests: astromech-data recorded.rs exact/neutron, astromech-galaxy tests/plot.rs, astromech-ship fsd_for_route_plotters, astromech-core plotted_routes_load_into_the_follower and copy.rs), check-deps, check-guide, shell cargo metadata --locked, and in app/ pnpm lint, check, build, test (View.test.ts plots and follows). Not covered: cargo-deny (not installed; CI runs it) and the Tauri shell build (Linux host lacks glib; CI Windows job)."
updated_at: "2026-10-17T19:11:47.092886457+00:00"
---

## Summary

All gates at HEAD: cargo fmt --check, clippy -D warnings, cargo test --workspace (incl. recorded Spansh tests: astromech-data recorded.rs exact/neutron, astromech-galaxy tests/plot.rs, astromech-ship fsd_for_route_plotters, astromech-core plotted_routes_load_into_the_follower and copy.rs), check-deps, check-guide, shell cargo metadata --locked, and in app/ pnpm lint, check, build, test (View.test.ts plots and follows). Not covered: cargo-deny (not installed; CI runs it) and the Tauri shell build (Linux host lacks glib; CI Windows job).

## Command

```console
bash -c 'set -eo pipefail; export CARGO_BUILD_JOBS=3; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | tail -n 300; python3 scripts/check-deps.py; python3 scripts/check-guide.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo metadata-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'
```

Exit status: 0

## Stdout

Bytes: 18473
Truncated: yes

```text

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s


running 10 tests
..........
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
..............................
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 24 tests
........................
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 17 tests
.................
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


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


running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s


running 32 tests
................................
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 30 tests
..............................
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 fail
```

## Stderr

Bytes: 60
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
```

