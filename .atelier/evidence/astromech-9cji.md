---
created_at: "2026-10-16T19:22:02.749000317+00:00"
id: "astromech-9cji"
evidence_type: "test"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:21:59.583649243+00:00"
command: "bash -c 'cd app && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cd app && pnpm test'"
updated_at: "2026-10-16T19:22:02.749324382+00:00"
---

## Summary

bash -c 'cd app && pnpm test'

## Command

```console
bash -c 'cd app && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 344
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-z4jb/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-z4jb/app


 Test Files  1 passed (1)
      Tests  6 passed (6)
   Start at  15:22:00
   Duration  2.33s (tests 56%, environment 28%, setup 8%, transform 7%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

