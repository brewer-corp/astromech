---
created_at: "2026-10-17T00:10:04.941350403+00:00"
id: "astromech-cuny"
evidence_type: "test"
commit: "81ea5c01089a72424ed348ea940ad595dd6f61a1"
result: "pass"
captured_at: "2026-10-17T00:09:46.172832840+00:00"
command: "bash -c 'cd app && pnpm install --frozen-lockfile >/dev/null && pnpm lint >/dev/null && pnpm check >/dev/null && pnpm test 2>&1 | grep -E '\"'\"'Test Files|Tests '\"'\"''"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6xyo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6xyo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main: app pnpm install --frozen-lockfile, lint, check, test"
updated_at: "2026-10-17T00:10:04.941639994+00:00"
---

## Summary

After merging main: app pnpm install --frozen-lockfile, lint, check, test

## Command

```console
bash -c 'cd app && pnpm install --frozen-lockfile >/dev/null && pnpm lint >/dev/null && pnpm check >/dev/null && pnpm test 2>&1 | grep -E '"'"'Test Files|Tests '"'"''
```

Exit status: 0

## Stdout

Bytes: 58
Truncated: no

```text
 Test Files  29 passed (29)
      Tests  136 passed (136)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

