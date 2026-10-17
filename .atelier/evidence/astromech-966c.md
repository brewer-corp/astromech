---
created_at: "2026-10-17T19:34:29.203554974+00:00"
id: "astromech-966c"
evidence_type: "test"
commit: "abdd88913b9c6888cc897fd020e8a91ae21408b8"
result: "pass"
captured_at: "2026-10-17T19:34:26.928179598+00:00"
command: "sh -c 'cd app && pnpm test src/lib/fakecore.contract'"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-c9em"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-c9em"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "sh -c 'cd app && pnpm test src/lib/fakecore.contract'"
updated_at: "2026-10-17T19:34:29.203912811+00:00"
---

## Summary

sh -c 'cd app && pnpm test src/lib/fakecore.contract'

## Command

```console
sh -c 'cd app && pnpm test src/lib/fakecore.contract'
```

Exit status: 0

## Stdout

Bytes: 327
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-config/app
> vitest run src/lib/fakecore.contract


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-config/app


 Test Files  1 passed (1)
      Tests  1 passed (1)
   Start at  15:34:27
   Duration  1.49s (tests 55%, environment 31%, setup 8%, transform 6%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

