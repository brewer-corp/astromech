---
created_at: "2026-11-05T16:00:41.559778974+00:00"
id: "astromech-lmf9"
evidence_type: "test"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:00:10.299504819+00:00"
command: "bash -c 'cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "validator-fable-kyqw"
target:
  kind: "issue"
  id: "astromech-kyqw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kyqw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "App checks run fresh on feature/astromech-51ue (in app/: pnpm lint && pnpm check && pnpm build && pnpm test; vitest has no skipped tests)"
updated_at: "2026-11-05T16:00:41.560476555+00:00"
---

## Summary

App checks run fresh on feature/astromech-51ue (in app/: pnpm lint && pnpm check && pnpm build && pnpm test; vitest has no skipped tests)

## Command

```console
bash -c 'cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 10431
SHA-256: 60748d6f350129aeef872e4fc88f6b0a860dacfc64961b21c55e3d00f6bb1bf8

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
