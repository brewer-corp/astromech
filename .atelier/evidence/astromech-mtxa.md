---
created_at: "2026-10-16T21:55:28.660273631+00:00"
id: "astromech-mtxa"
evidence_type: "test"
commit: "7bdf6080b1ede3015d034190cb9c5e00c553fb1e"
result: "pass"
captured_at: "2026-10-16T21:55:08.677380926+00:00"
command: "bash -c 'cd app && pnpm -s lint && pnpm -s check && pnpm -s build >/dev/null && pnpm -s test'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-osjr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-osjr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "App gates: lint, svelte-check, build, vitest (Sites.test.ts: shards rows with jumps/arrival/landmark position, kind switch to known farms)"
updated_at: "2026-10-16T21:55:28.660674199+00:00"
---

## Summary

App gates: lint, svelte-check, build, vitest (Sites.test.ts: shards rows with jumps/arrival/landmark position, kind switch to known farms)

## Command

```console
bash -c 'cd app && pnpm -s lint && pnpm -s check && pnpm -s build >/dev/null && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 404
Truncated: no

```text
1792187718357 START "/home/supadava/projects/astromech-wt-feature-astromech-sfys/app"
1792187718359 COMPLETED 271 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-sfys/app


 Test Files  12 passed (12)
      Tests  67 passed (67)
   Start at  17:55:21
   Duration  6.64s (tests 44%, environment 38%, transform 9%, setup 8%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

