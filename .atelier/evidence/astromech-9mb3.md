---
created_at: "2026-10-17T19:25:02.647387320+00:00"
id: "astromech-9mb3"
evidence_type: "test"
commit: "64215b2eb60fc7ed795691cbee157c98a81f9478"
result: "pass"
captured_at: "2026-10-17T19:24:46.732973398+00:00"
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
updated_at: "2026-10-17T19:25:02.647822772+00:00"
---

## Summary

pnpm --dir app test -- controller

## Command

```console
pnpm --dir app test -- controller
```

Exit status: 0

## Stdout

Bytes: 632
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-voice/app
> vitest run -- controller


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-voice/app


 Test Files  46 passed (46)
      Tests  210 passed (210)
   Start at  15:24:48
   Duration  14.24s (environment 45%, tests 39%, setup 10%, transform 4%, import 2%)

Environment  jsdom was created 46 times · 61.68s total, 45% of tracked time
             create it once per worker with pool: 'vmThreads' (keeps per-file isolation) or isolate: false (shares it across files)
             learn more: https://vitest.dev/guide/improving-performance#test-environments
```

## Stderr

Bytes: 0
Truncated: no

```text
```

