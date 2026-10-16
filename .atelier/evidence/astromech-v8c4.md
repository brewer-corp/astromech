---
created_at: "2026-10-16T23:35:37.553877036+00:00"
id: "astromech-v8c4"
evidence_type: "test"
commit: "ba77ef85bf23f20da5764d5aa18a93113fe17139"
result: "pass"
captured_at: "2026-10-16T23:35:35.447758648+00:00"
command: "bash -c 'cd app && pnpm exec vitest run src/modules/trade src/lib/trade.test.ts'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mljg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mljg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Route mode component tests: defaults from the ship, single hop and loop shown with cr/t and estimated cr/h, overrides passed to the lookup, no route, empty way back, Spansh error, disabled without hold or credits"
updated_at: "2026-10-16T23:35:37.554303342+00:00"
---

## Summary

Route mode component tests: defaults from the ship, single hop and loop shown with cr/t and estimated cr/h, overrides passed to the lookup, no route, empty way back, Spansh error, disabled without hold or credits

## Command

```console
bash -c 'cd app && pnpm exec vitest run src/modules/trade src/lib/trade.test.ts'
```

Exit status: 0

## Stdout

Bytes: 237
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-mljg/app


 Test Files  2 passed (2)
      Tests  7 passed (7)
   Start at  19:35:36
   Duration  1.16s (environment 51%, tests 24%, setup 12%, transform 11%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

