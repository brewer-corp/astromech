---
created_at: "2026-10-17T00:04:08.060537288+00:00"
id: "astromech-j87h"
evidence_type: "test"
commit: "e3cd7deb8351358554dcfecde039bb0fd64e9a96"
result: "pass"
captured_at: "2026-10-17T00:03:53.640672372+00:00"
command: "bash -c 'cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm test'"
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
title: "App gates: pnpm install --frozen-lockfile, lint, check, test"
updated_at: "2026-10-17T00:04:08.060977220+00:00"
---

## Summary

App gates: pnpm install --frozen-lockfile, lint, check, test

## Command

```console
bash -c 'cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 1161
Truncated: no

```text
Lockfile is up to date, resolution step is skipped
Already up to date

Done in 572ms using pnpm v10.34.6

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-task-astromech-6xyo/app
> eslint .


> astromech-app@0.1.5 check /home/supadava/projects/astromech-wt-task-astromech-6xyo/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792195441016 START "/home/supadava/projects/astromech-wt-task-astromech-6xyo/app"
1792195441018 COMPLETED 337 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-task-astromech-6xyo/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-6xyo/app


 Test Files  26 passed (26)
      Tests  109 passed (109)
   Start at  20:04:01
   Duration  6.23s (environment 43%, tests 40%, setup 9%, transform 5%, import 1%)

Environment  jsdom was created 26 times · 21.83s total, 43% of tracked time
             create it once per worker with pool: 'vmThreads' (keeps per-file isolation) or isolate: false (shares it across files)
             learn more: https://vitest.dev/guide/improving-performance#test-environments
```

## Stderr

Bytes: 0
Truncated: no

```text
```

