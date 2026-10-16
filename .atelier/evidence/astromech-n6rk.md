---
created_at: "2026-10-16T20:04:32.450206219+00:00"
id: "astromech-n6rk"
evidence_type: "test"
commit: "f28ddf221fdd3339c2e0e6b2a19dbe868e824618"
result: "pass"
captured_at: "2026-10-16T20:03:56.203693315+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -q -- -D warnings && cargo test --workspace -q >/dev/null && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm -s lint && pnpm -s check >/dev/null && pnpm -s build >/dev/null && pnpm -s test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-orel"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-orel"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R5 and full checks: hand-edited speeds show as the nearest step (fails without the clamp); workspace tests, clippy -D warnings, dependency rules, shell --locked metadata, app lint/check/build/test"
updated_at: "2026-10-16T20:04:32.450692166+00:00"
---

## Summary

R5 and full checks: hand-edited speeds show as the nearest step (fails without the clamp); workspace tests, clippy -D warnings, dependency rules, shell --locked metadata, app lint/check/build/test

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -q -- -D warnings && cargo test --workspace -q >/dev/null && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm -s lint && pnpm -s check >/dev/null && pnpm -s build >/dev/null && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 282
Truncated: no

```text
dependency rules OK (16 workspace crates)

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-lxma/app


 Test Files  6 passed (6)
      Tests  40 passed (40)
   Start at  16:04:25
   Duration  6.41s (tests 54%, environment 28%, transform 8%, setup 8%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

