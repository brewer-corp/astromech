---
created_at: "2026-10-22T21:25:13.746263509+00:00"
id: "astromech-v7zi"
evidence_type: "test"
commit: "8c11bfef29e9de07d0b94b7a7576847ad3111b24"
result: "pass"
captured_at: "2026-10-22T21:25:08.677069750+00:00"
command: "pnpm -C app exec vitest run src/modules/engineering/Build.test.ts src/modules/settings/View.test.ts src/lib/onboarding src/modules/trade/Route.test.ts"
exit_status: "0"
agent_identity: "claude-implementer-jo3l"
target:
  kind: "issue"
  id: "astromech-cvxf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-cvxf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Engineer progress, Settings and setup status, and trade route defaults show their failures (tests fail without the change)"
updated_at: "2026-10-22T21:25:13.746520057+00:00"
---

## Summary

Engineer progress, Settings and setup status, and trade route defaults show their failures (tests fail without the change)

## Command

```console
pnpm -C app exec vitest run src/modules/engineering/Build.test.ts src/modules/settings/View.test.ts src/lib/onboarding src/modules/trade/Route.test.ts
```

Exit status: 0

## Stdout

Bytes: 232
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt/astromech-jo3l/app


 Test Files  4 passed (4)
      Tests  47 passed (47)
   Start at  17:25:09
   Duration  4.31s (tests 77%, environment 15%, transform 4%, setup 4%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

