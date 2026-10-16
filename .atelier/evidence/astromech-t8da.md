---
created_at: "2026-10-16T19:25:06.949802233+00:00"
id: "astromech-t8da"
evidence_type: "test"
commit: "cbcfc2475678431decf542e4d0775b99a895675e"
result: "pass"
captured_at: "2026-10-16T19:24:53.859561038+00:00"
command: "sh -c 'cd app && pnpm -s lint && pnpm -s check && pnpm -s build >/dev/null && pnpm -s test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mmqs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mmqs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "App checks after the review fixes"
updated_at: "2026-10-16T19:25:06.950016794+00:00"
---

## Summary

App checks after the review fixes

## Command

```console
sh -c 'cd app && pnpm -s lint && pnpm -s check && pnpm -s build >/dev/null && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 402
Truncated: no

```text
1792178700665 START "/home/supadava/projects/astromech-wt-feature-astromech-lxma/app"
1792178700667 COMPLETED 246 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-lxma/app


 Test Files  2 passed (2)
      Tests  18 passed (18)
   Start at  15:25:04
   Duration  2.68s (tests 54%, environment 33%, setup 7%, transform 5%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

