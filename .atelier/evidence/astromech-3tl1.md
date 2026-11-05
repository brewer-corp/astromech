---
created_at: "2026-11-04T21:05:53.447867942+00:00"
id: "astromech-3tl1"
evidence_type: "test"
commit: "655647a5b127e6319f3d5787405a6b284d105301"
result: "pass"
captured_at: "2026-11-04T21:05:47.671082972+00:00"
command: "npx vitest run src/lib/onboarding"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-9kp2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-9kp2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Onboarding.test.ts astromech-9kp2: rows say 'Checking…' while the first core_status never answers, 'Could not read' when every read failed, then the buttons once a read succeeds; never 'Not set'. Both fail on the old components"
updated_at: "2026-11-04T21:05:53.448140330+00:00"
---

## Summary

Onboarding.test.ts astromech-9kp2: rows say 'Checking…' while the first core_status never answers, 'Could not read' when every read failed, then the buttons once a read succeeds; never 'Not set'. Both fail on the old components

## Command

```console
npx vitest run src/lib/onboarding
```

Exit status: 0

## Stdout

Bytes: 206
SHA-256: 7630ac6c16b54439fce814eecc6d0af4ab06b5ee262802f49869f9097cae4314

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
