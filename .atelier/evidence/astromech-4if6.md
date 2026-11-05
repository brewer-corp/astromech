---
created_at: "2026-11-05T01:17:43.467354921+00:00"
id: "astromech-4if6"
evidence_type: "test"
commit: "c39a9e18a005ecd28104ae105297f2314b6c30bc"
result: "pass"
captured_at: "2026-11-05T01:16:55.912139738+00:00"
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
updated_at: "2026-11-05T01:17:43.467762291+00:00"
---

## Summary

pnpm check and pnpm test on the branch merged with main

## Command

```console
sh -c 'pnpm -s check && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 741
SHA-256: e8c8be6733d94299655515551dec23f20da84e8419158846db64767bcf83dd29

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
