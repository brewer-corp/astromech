---
created_at: "2026-10-17T02:49:15.068049271+00:00"
id: "astromech-9q34"
evidence_type: "test"
commit: "81ec4a045bd1bebd95453b4f41c32e9d39f396df"
result: "pass"
captured_at: "2026-10-17T02:49:12.554904777+00:00"
command: "bash -c 'cd app && pnpm vitest run src/modules/station'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-qw2m"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qw2m"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes: station Market component tests incl. 'Market as of' age (R2) and 'Not looked up' lines"
updated_at: "2026-10-17T02:49:15.068482800+00:00"
---

## Summary

After review fixes: station Market component tests incl. 'Market as of' age (R2) and 'Not looked up' lines

## Command

```console
bash -c 'cd app && pnpm vitest run src/modules/station'
```

Exit status: 0

## Stdout

Bytes: 236
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-qw2m/app


 Test Files  2 passed (2)
      Tests  8 passed (8)
   Start at  22:49:13
   Duration  1.39s (environment 44%, tests 37%, setup 10%, transform 8%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

