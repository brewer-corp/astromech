---
created_at: "2026-10-16T19:35:30.977415780+00:00"
id: "astromech-3btd"
evidence_type: "test"
commit: "27e5828139abb95e372a951c42fe8ea13d905cc0"
result: "pass"
captured_at: "2026-10-16T19:35:26.520886678+00:00"
command: "bash -c 'cd app && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-9243"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-9243"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cd app && pnpm test'"
updated_at: "2026-10-16T19:35:30.977798022+00:00"
---

## Summary

bash -c 'cd app && pnpm test'

## Command

```console
bash -c 'cd app && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 357
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-z4jb/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-z4jb/app


 Test Files  2 passed (2)
      Tests  9 passed (9)
   Start at  15:35:27
   Duration  3.48s (tests 37%, environment 36%, transform 12%, setup 12%, import 2%, worker 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

