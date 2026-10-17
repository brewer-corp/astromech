---
created_at: "2026-10-17T02:46:16.686788799+00:00"
id: "astromech-vquk"
evidence_type: "test"
commit: "09fe9582eb59b2b43dd660eb6787cc5b134da783"
result: "pass"
captured_at: "2026-10-17T02:46:13.146938901+00:00"
command: "bash -c 'set -eo pipefail; cd app && pnpm vitest run src/lib/ui/Colonies.test.ts src/modules/colonisation'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-20dy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-20dy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Component tests: Colonies kit (interest tabs, rows, radius default) and the Colonisation page showing candidates without projects"
updated_at: "2026-10-17T02:46:16.687082917+00:00"
---

## Summary

Component tests: Colonies kit (interest tabs, rows, radius default) and the Colonisation page showing candidates without projects

## Command

```console
bash -c 'set -eo pipefail; cd app && pnpm vitest run src/lib/ui/Colonies.test.ts src/modules/colonisation'
```

Exit status: 0

## Stdout

Bytes: 237
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-20dy/app


 Test Files  2 passed (2)
      Tests  9 passed (9)
   Start at  22:46:14
   Duration  2.10s (environment 49%, tests 29%, setup 10%, transform 10%, import 2%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

