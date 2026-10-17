---
created_at: "2026-10-17T00:27:27.662470020+00:00"
id: "astromech-4te4"
evidence_type: "test"
commit: "467ae488f07bd2be3d1861f435d8e3cf0e166a86"
result: "pass"
captured_at: "2026-10-17T00:27:24.805558096+00:00"
command: "bash -c 'cd app && pnpm vitest run src/lib/outfitting.test.ts'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-970a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-970a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R6 app: picker offers armour and pre-engineered modules, no Basic Discovery Scanner, routes resolve; failed before (2 tests)"
updated_at: "2026-10-17T00:27:27.662945188+00:00"
---

## Summary

R6 app: picker offers armour and pre-engineered modules, no Basic Discovery Scanner, routes resolve; failed before (2 tests)

## Command

```console
bash -c 'cd app && pnpm vitest run src/lib/outfitting.test.ts'
```

Exit status: 0

## Stdout

Bytes: 236
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-970a/app


 Test Files  1 passed (1)
      Tests  9 passed (9)
   Start at  20:27:26
   Duration  1.46s (environment 67%, setup 17%, transform 14%, tests 2%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

