---
created_at: "2026-10-16T23:20:34.921775832+00:00"
id: "astromech-ebt8"
evidence_type: "test"
commit: "f219f91bfddf01b3ccbe1d7a4046ff179bae2a31"
result: "pass"
captured_at: "2026-10-16T23:20:05.879983905+00:00"
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
title: "Full gates on f219f91 (review fixes 9293 empty-hold Cargo.json, t8cf poll sequencing and Cargo.json-bytes cache key): cargo fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, app pnpm install --frozen-lockfile, lint, check, build, test. Shell change type-checked in the scratch crate."
updated_at: "2026-10-16T23:20:34.922233586+00:00"
---

## Summary

Full gates on f219f91 (review fixes 9293 empty-hold Cargo.json, t8cf poll sequencing and Cargo.json-bytes cache key): cargo fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, app pnpm install --frozen-lockfile, lint, check, build, test. Shell change type-checked in the scratch crate.

## Command

```console
bash -c 'set -e; cargo fmt --all --check; cargo clippy --workspace --all-targets -q -- -D warnings; cargo test --workspace -q 2>&1 | grep -E "^test result" | awk "{p+=\$4; f+=\$6} END {print \"rust passed=\"p\" failed=\"f; exit f>0}"; python3 scripts/check-deps.py; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo shell-lock-ok); cd app; pnpm install --frozen-lockfile >/dev/null; pnpm lint; pnpm check | tail -1; pnpm build | tail -1; pnpm test 2>&1 | grep -E "Test Files|Tests "'
```

Exit status: 0

## Stdout

Bytes: 336
Truncated: no

```text
rust passed=486 failed=0
dependency rules OK (23 workspace crates)
shell-lock-ok

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-feature-astromech-k3wp/app
> eslint .

1792192825115 COMPLETED 302 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
✓ built in 2.04s
 Test Files  19 passed (19)
      Tests  90 passed (90)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

