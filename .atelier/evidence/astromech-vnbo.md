---
created_at: "2026-10-16T18:37:58.812915933+00:00"
id: "astromech-vnbo"
evidence_type: "test"
commit: "f8fad79d3bc29e98c18dac466437a08d267b0e76"
result: "pass"
captured_at: "2026-10-16T18:37:52.751147109+00:00"
command: "bash -c 'cd app && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-g5rw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-g5rw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Component tests: All lists every binding on the device; an activity narrows the list and shows unbound gaps with Bind this"
updated_at: "2026-10-16T18:37:58.813384496+00:00"
---

## Summary

Component tests: All lists every binding on the device; an activity narrows the list and shows unbound gaps with Bind this

## Command

```console
bash -c 'cd app && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 357
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-q318/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-q318/app


 Test Files  2 passed (2)
      Tests  6 passed (6)
   Start at  14:37:54
   Duration  4.46s (tests 37%, environment 32%, transform 15%, setup 14%, import 2%, worker 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

