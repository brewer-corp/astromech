---
created_at: "2026-10-22T21:24:40.418791066+00:00"
id: "astromech-w6w6"
evidence_type: "test"
commit: "8c11bfef29e9de07d0b94b7a7576847ad3111b24"
result: "pass"
captured_at: "2026-10-22T21:24:36.995895822+00:00"
command: "pnpm -C app exec vitest run src/lib/ui/staleTauri.test.ts src/lib/mining.test.ts"
exit_status: "0"
agent_identity: "claude-implementer-jo3l"
target:
  kind: "issue"
  id: "astromech-wv9x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-wv9x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Mining in the app: core up with no session says 'No mining run yet.'; core down or shell failing says 'Could not read the mining run: the core did not answer.' (fail when the old text is restored)"
updated_at: "2026-10-22T21:24:40.419114479+00:00"
---

## Summary

Mining in the app: core up with no session says 'No mining run yet.'; core down or shell failing says 'Could not read the mining run: the core did not answer.' (fail when the old text is restored)

## Command

```console
pnpm -C app exec vitest run src/lib/ui/staleTauri.test.ts src/lib/mining.test.ts
```

Exit status: 0

## Stdout

Bytes: 234
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt/astromech-jo3l/app


 Test Files  2 passed (2)
      Tests  19 passed (19)
   Start at  17:24:38
   Duration  1.51s (environment 51%, tests 19%, setup 14%, transform 14%, import 3%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

