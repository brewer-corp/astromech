---
created_at: "2026-10-17T17:28:23.068889206+00:00"
id: "astromech-eqnt"
evidence_type: "test"
commit: "eb6346ea4377eb9b4e16a8695e633d5b15bd25bd"
result: "pass"
captured_at: "2026-10-17T17:28:20.002721676+00:00"
command: "sh -c 'cd app && pnpm vitest run src/lib/help.test.ts src/App.test.ts src/lib/onboarding'"
exit_status: "0"
agent_identity: "docs-agent"
target:
  kind: "issue"
  id: "astromech-basj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-basj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "sh -c 'cd app && pnpm vitest run src/lib/help.test.ts src/App.test.ts src/lib/onboarding'"
updated_at: "2026-10-17T17:28:23.069146696+00:00"
---

## Summary

sh -c 'cd app && pnpm vitest run src/lib/help.test.ts src/App.test.ts src/lib/onboarding'

## Command

```console
sh -c 'cd app && pnpm vitest run src/lib/help.test.ts src/App.test.ts src/lib/onboarding'
```

Exit status: 0

## Stdout

Bytes: 222
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-docs/app


 Test Files  3 passed (3)
      Tests  29 passed (29)
   Start at  13:28:20
   Duration  2.10s (tests 54%, environment 29%, transform 8%, setup 7%, import 2%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

