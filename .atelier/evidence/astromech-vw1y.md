---
created_at: "2026-10-17T02:14:13.867580708+00:00"
id: "astromech-vw1y"
evidence_type: "test"
commit: "a609322196103ec3e427a5261247651e92393f56"
result: "pass"
captured_at: "2026-10-17T02:13:41.112337054+00:00"
command: "bash -c 'set -e; export CARGO_BUILD_JOBS=6; cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace 2>&1 | grep -E \"test result|FAILED\"; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo metadata-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'"
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
title: "All gates after the review fixes (9f94511) and the merge of main (a609322): cargo fmt --check, clippy -D warnings, cargo test --workspace (651 passed), check-deps, shell cargo metadata --locked --offline, and in app/ pnpm install --frozen-lockfile, lint, check, build, test (151 passed)."
updated_at: "2026-10-17T02:14:13.868154058+00:00"
---

## Summary

All gates after the review fixes (9f94511) and the merge of main (a609322): cargo fmt --check, clippy -D warnings, cargo test --workspace (651 passed), check-deps, shell cargo metadata --locked --offline, and in app/ pnpm install --frozen-lockfile, lint, check, build, test (151 passed).

## Command

```console
bash -c 'set -e; export CARGO_BUILD_JOBS=6; cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace 2>&1 | grep -E "test result|FAILED"; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo metadata-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'
```

Exit status: 0

## Stdout

Bytes: 17849
Truncated: yes

```text
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 89 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.43s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.52s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.67s
test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 108 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.62s
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 5 passed; 0 failed; 0
```

## Stderr

Bytes: 72
Truncated: no

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

