---
created_at: "2026-10-17T01:03:22.585119048+00:00"
id: "astromech-xz14"
evidence_type: "test"
commit: "1ffbef839da4ec7ab4d15a962d9340306798d37a"
result: "pass"
captured_at: "2026-10-17T01:03:06.878440614+00:00"
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
title: "After merging main (5t34): app pnpm install --frozen-lockfile, lint, check, test"
updated_at: "2026-10-17T01:03:22.585357903+00:00"
---

## Summary

After merging main (5t34): app pnpm install --frozen-lockfile, lint, check, test

## Command

```console
bash -c 'cd app && pnpm install --frozen-lockfile >/dev/null && pnpm lint >/dev/null && pnpm check >/dev/null && pnpm test 2>&1 | grep -E '"'"'Test Files|Tests '"'"''
```

Exit status: 0

## Stdout

Bytes: 58
Truncated: no

```text
 Test Files  32 passed (32)
      Tests  145 passed (145)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

