---
created_at: "2026-10-16T19:24:53.576820816+00:00"
id: "astromech-2jwy"
evidence_type: "test"
commit: "cbcfc2475678431decf542e4d0775b99a895675e"
result: "pass"
captured_at: "2026-10-16T19:24:53.309968896+00:00"
command: "sh -c 'cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mmqs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mmqs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Shell builds --locked after the review fixes"
updated_at: "2026-10-16T19:24:53.577218939+00:00"
---

## Summary

Shell builds --locked after the review fixes

## Command

```console
sh -c 'cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 0
Truncated: no

```text
```

