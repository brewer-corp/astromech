---
created_at: "2026-10-16T19:55:47.772017353+00:00"
id: "astromech-mwh4"
evidence_type: "test"
commit: "49fd4da658c9b34062477dae76127643b814b925"
result: "pass"
captured_at: "2026-10-16T19:55:16.385867223+00:00"
command: "sh -c 'cargo test --workspace -q >/dev/null && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm -s lint && pnpm -s check >/dev/null && pnpm -s build >/dev/null && pnpm -s test'"
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
title: "Full checks after the merge and orel: workspace tests, dependency rules, shell --locked metadata, app lint/check/build/test"
updated_at: "2026-10-16T19:55:47.772441114+00:00"
---

## Summary

Full checks after the merge and orel: workspace tests, dependency rules, shell --locked metadata, app lint/check/build/test

## Command

```console
sh -c 'cargo test --workspace -q >/dev/null && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm -s lint && pnpm -s check >/dev/null && pnpm -s build >/dev/null && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 282
Truncated: no

```text
dependency rules OK (16 workspace crates)

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-lxma/app


 Test Files  6 passed (6)
      Tests  37 passed (37)
   Start at  15:55:42
   Duration  5.62s (tests 61%, environment 26%, setup 6%, transform 6%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

