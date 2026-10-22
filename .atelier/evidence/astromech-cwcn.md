---
created_at: "2026-10-22T21:24:57.589169942+00:00"
id: "astromech-cwcn"
evidence_type: "test"
commit: "8c11bfef29e9de07d0b94b7a7576847ad3111b24"
result: "pass"
captured_at: "2026-10-22T21:24:55.470671718+00:00"
command: "pnpm -C app exec vitest run src/lib/fakecore.preview.test.ts"
exit_status: "0"
agent_identity: "claude-implementer-jo3l"
target:
  kind: "issue"
  id: "astromech-95f6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-95f6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Nearby, Sites, Colonies and Stations lookups under ?core=empty reject with 'no position in the journal yet' without a typed system and answer from a typed one; ?fail= still applies. Screenshots in docs/evidence/astromech-95f6"
updated_at: "2026-10-22T21:24:57.589480871+00:00"
---

## Summary

Nearby, Sites, Colonies and Stations lookups under ?core=empty reject with 'no position in the journal yet' without a typed system and answer from a typed one; ?fail= still applies. Screenshots in docs/evidence/astromech-95f6

## Command

```console
pnpm -C app exec vitest run src/lib/fakecore.preview.test.ts
```

Exit status: 0

## Stdout

Bytes: 234
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt/astromech-jo3l/app


 Test Files  1 passed (1)
      Tests  19 passed (19)
   Start at  17:24:56
   Duration  1.18s (environment 53%, tests 19%, setup 14%, transform 12%, import 2%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

