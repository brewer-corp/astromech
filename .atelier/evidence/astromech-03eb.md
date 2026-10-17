---
created_at: "2026-10-17T02:21:58.749296923+00:00"
id: "astromech-03eb"
evidence_type: "test"
commit: "be0ce7d8a17ac6a9d7249d247761d282c2ae1ae5"
result: "pass"
captured_at: "2026-10-17T02:21:25.391206968+00:00"
command: "bash -c 'set -eo pipefail; export CARGO_BUILD_JOBS=6; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | tail -n 400; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo metadata-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-77a1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-77a1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "All gates after the l9dh fix (6a476e0) and the second merge of main (be0ce7d), exit status covering every step (pipefail): cargo fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, and in app/ pnpm install --frozen-lockfile, lint, check, build, test."
updated_at: "2026-10-17T02:21:58.749635163+00:00"
---

## Summary

All gates after the l9dh fix (6a476e0) and the second merge of main (be0ce7d), exit status covering every step (pipefail): cargo fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, and in app/ pnpm install --frozen-lockfile, lint, check, build, test.

## Command

```console
bash -c 'set -eo pipefail; export CARGO_BUILD_JOBS=6; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | tail -n 400; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo metadata-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'
```

Exit status: 0

## Stdout

Bytes: 19835
Truncated: yes

```text
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
..........
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 18 tests
..................
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 89 tests
....................................................................................... 87/89
..
test result: ok. 89 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.33s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.52s


running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.68s


running 6 tests
i.....
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 14 tests
..............
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 108 tests
....................................................................................... 87/108
.....................
test result: ok. 108 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.62s


running 24 tests
........................
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 38 tests
......................................
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 18 tests
..................
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s


running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 29 tests
.............................
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 24 tests
........................
test result: ok. 24 passed; 0 failed; 0 ignored; 0 me
```

## Stderr

Bytes: 0
Truncated: no

```text
```

