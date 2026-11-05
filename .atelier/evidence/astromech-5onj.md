---
created_at: "2026-11-05T00:56:48.568174796+00:00"
id: "astromech-5onj"
evidence_type: "test"
commit: "3b74c7d0b6ec99d84e177f18be50b53663639a11"
result: "pass"
captured_at: "2026-11-05T00:56:28.634079583+00:00"
command: "sh -c 'pnpm -s check && pnpm -s test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-c4i0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-c4i0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "pnpm check and pnpm test on the branch merged with main"
updated_at: "2026-11-05T00:56:48.568458835+00:00"
---

## Summary

pnpm check and pnpm test on the branch merged with main

## Command

```console
sh -c 'pnpm -s check && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 740
SHA-256: 8568d1c85438b51000272d99e6c84a1591c1cb43f1be9dc1b7aa576f1ed0fd92

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
