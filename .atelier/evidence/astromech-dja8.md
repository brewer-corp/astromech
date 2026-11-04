---
created_at: "2026-11-04T21:11:37.322296378+00:00"
id: "astromech-dja8"
evidence_type: "test"
commit: "005676b27ca92e6ab6db0597e04694179a4023e3"
result: "pass"
captured_at: "2026-11-04T21:11:34.713605433+00:00"
command: "npx vitest run src/modules/settings/announce.test.ts src/lib/ui/announce.test.ts"
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
title: "After the R1 fix: announce.test.ts (component: text, empty, text again on repeats) and lib/ui/announce.test.ts (region stays empty a frame later, then filled; newer call wins) pass"
updated_at: "2026-11-04T21:11:37.322559288+00:00"
---

## Summary

After the R1 fix: announce.test.ts (component: text, empty, text again on repeats) and lib/ui/announce.test.ts (region stays empty a frame later, then filled; newer call wins) pass

## Command

```console
npx vitest run src/modules/settings/announce.test.ts src/lib/ui/announce.test.ts
```

Exit status: 0

## Stdout

Bytes: 217
SHA-256: 46952c48087596971821c1b94d442535fdc624b7ec30f251cb4b410f97044c0d

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
