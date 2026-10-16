---
created_at: "2026-10-16T23:04:14.736687463+00:00"
id: "astromech-jlgd"
evidence_type: "test"
commit: "2e9bc9387a1f0325594aeb596114eb021636e7e8"
result: "pass"
captured_at: "2026-10-16T23:03:47.322730631+00:00"
command: "bash -c 'set -e; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | grep -E \"^test result\" | awk \"{p+=\\$4; f+=\\$6} END {print \\\"rust passed=\\\"p\\\" failed=\\\"f; exit f>0}\"; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo shell-lock-ok); cd app; pnpm install --frozen-lockfile >/dev/null; pnpm lint; pnpm check | tail -1; pnpm build | tail -1; pnpm test 2>&1 | grep -E \"Test Files|Tests \"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-05bi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-05bi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full gates on 2e9bc93: cargo fmt --check, clippy --workspace --all-targets -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, app pnpm install --frozen-lockfile, lint, check, build, test"
updated_at: "2026-10-16T23:04:14.737107747+00:00"
---

## Summary

Full gates on 2e9bc93: cargo fmt --check, clippy --workspace --all-targets -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, app pnpm install --frozen-lockfile, lint, check, build, test

## Command

```console
bash -c 'set -e; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | grep -E "^test result" | awk "{p+=\$4; f+=\$6} END {print \"rust passed=\"p\" failed=\"f; exit f>0}"; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo shell-lock-ok); cd app; pnpm install --frozen-lockfile >/dev/null; pnpm lint; pnpm check | tail -1; pnpm build | tail -1; pnpm test 2>&1 | grep -E "Test Files|Tests "'
```

Exit status: 0

## Stdout

Bytes: 336
Truncated: no

```text
rust passed=485 failed=0
dependency rules OK (23 workspace crates)
shell-lock-ok

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-feature-astromech-k3wp/app
> eslint .

1792191845562 COMPLETED 302 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
✓ built in 1.49s
 Test Files  19 passed (19)
      Tests  88 passed (88)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

