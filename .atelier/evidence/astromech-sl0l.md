---
created_at: "2026-10-22T21:25:00.155044074+00:00"
id: "astromech-sl0l"
evidence_type: "test"
commit: "8c11bfef29e9de07d0b94b7a7576847ad3111b24"
result: "pass"
captured_at: "2026-10-22T21:24:57.785774667+00:00"
command: "pnpm -C app exec vitest run src/lib/ui/staleMocked.test.ts src/modules/route"
exit_status: "0"
agent_identity: "claude-implementer-jo3l"
target:
  kind: "issue"
  id: "astromech-v65y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-v65y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Route: a failed Skip keeps its error through three good polls and clears on the next action"
updated_at: "2026-10-22T21:25:00.155392002+00:00"
---

## Summary

Route: a failed Skip keeps its error through three good polls and clears on the next action

## Command

```console
pnpm -C app exec vitest run src/lib/ui/staleMocked.test.ts src/modules/route
```

Exit status: 0

## Stdout

Bytes: 233
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt/astromech-jo3l/app


 Test Files  3 passed (3)
      Tests  11 passed (11)
   Start at  17:24:58
   Duration  1.45s (environment 53%, tests 23%, setup 13%, transform 9%, import 2%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

