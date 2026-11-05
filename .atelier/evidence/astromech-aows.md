---
created_at: "2026-11-04T21:14:10.606848643+00:00"
id: "astromech-aows"
evidence_type: "test"
commit: "e9e444acb64e9fad88eb52f3620de576dc7614f2"
result: "pass"
captured_at: "2026-11-04T21:14:05.869606449+00:00"
command: "npx vitest run src/lib/ui/announce.test.ts src/modules/settings"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-aznm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-aznm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After R3: announce tests (per-region announcer; two regions at once both filled; newer call in one region wins) and the settings component tests pass"
updated_at: "2026-11-04T21:14:10.607112645+00:00"
---

## Summary

After R3: announce tests (per-region announcer; two regions at once both filled; newer call in one region wins) and the settings component tests pass

## Command

```console
npx vitest run src/lib/ui/announce.test.ts src/modules/settings
```

Exit status: 0

## Stdout

Bytes: 218
SHA-256: 96e8790b69cc326cacdfec11e859015138d40198e0d881946d3ea0d49fde51df

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
