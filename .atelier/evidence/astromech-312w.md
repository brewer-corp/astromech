---
created_at: "2026-10-17T19:29:47.256298366+00:00"
id: "astromech-312w"
evidence_type: "test"
commit: "ff32bfec92411960a3368009ddb40393ad6d854e"
result: "pass"
captured_at: "2026-10-17T19:29:29.664633473+00:00"
command: "pnpm --dir app test -- controller"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-54hd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-54hd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "pnpm --dir app test -- controller"
updated_at: "2026-10-17T19:29:47.256644261+00:00"
---

## Summary

pnpm --dir app test -- controller

## Command

```console
pnpm --dir app test -- controller
```

Exit status: 0

## Stdout

Bytes: 631
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-voice/app
> vitest run -- controller


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-voice/app


 Test Files  46 passed (46)
      Tests  211 passed (211)
   Start at  15:29:31
   Duration  15.30s (tests 40%, environment 40%, setup 9%, transform 9%, import 2%)

Environment  jsdom was created 46 times · 59.27s total, 40% of tracked time
             create it once per worker with pool: 'vmThreads' (keeps per-file isolation) or isolate: false (shares it across files)
             learn more: https://vitest.dev/guide/improving-performance#test-environments
```

## Stderr

Bytes: 0
Truncated: no

```text
```

