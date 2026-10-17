---
created_at: "2026-10-17T01:06:22.097181623+00:00"
id: "astromech-08vh"
evidence_type: "test"
commit: "b6ed86c8409f4b02d85992e59763626542f0e67b"
result: "pass"
captured_at: "2026-10-17T01:06:12.961663575+00:00"
command: "bash -c 'cd app && pnpm vitest run src/modules/powerplay --reporter=verbose && pnpm lint && pnpm check'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-w62j"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-w62j"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Component tests: Suggestions (order, ethos/weak spot/on-foot tags, quiet ? with screen-reader reason, key only when needed) and the Powerplay page panel states; app lint and typecheck clean"
updated_at: "2026-10-17T01:06:22.097471724+00:00"
---

## Summary

Component tests: Suggestions (order, ethos/weak spot/on-foot tags, quiet ? with screen-reader reason, key only when needed) and the Powerplay page panel states; app lint and typecheck clean

## Command

```console
bash -c 'cd app && pnpm vitest run src/modules/powerplay --reporter=verbose && pnpm lint && pnpm check'
```

Exit status: 0

## Stdout

Bytes: 2045
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-w62j/app

 ✓ src/modules/powerplay/Suggestions.test.ts > What earns merits here > lists each activity with how to do it, in the order given 38ms
 ✓ src/modules/powerplay/Suggestions.test.ts > What earns merits here > tags ethos, weak spots and on-foot activities 5ms
 ✓ src/modules/powerplay/Suggestions.test.ts > What earns merits here > marks unverified rules quietly and explains the mark 8ms
 ✓ src/modules/powerplay/Suggestions.test.ts > What earns merits here > leaves out the key when every rule is verified 2ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > shows the standing from the journal 76ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > says so when not pledged 6ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > shows the current system's Powerplay state 30ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > shows acquisition progress in an unoccupied system 15ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > shows what earns merits in the current system 15ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > says when nothing here earns the power merits 10ms
 ✓ src/modules/powerplay/View.test.ts > Powerplay page > says when the system has no Powerplay 11ms
 ✓ src/modules/powerplay/Targets.test.ts > Where to work > lists the pledged power's targets nearest first and re-asks on kind and range 167ms

 Test Files  3 passed (3)
      Tests  12 passed (12)
   Start at  21:06:13
   Duration  974ms (environment 57%, tests 15%, setup 14%, transform 12%, import 1%)


> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-task-astromech-w62j/app
> eslint .


> astromech-app@0.1.5 check /home/supadava/projects/astromech-wt-task-astromech-w62j/app
> svelte-check --tsconfig ./tsconfig.json --fail-on-warnings

1792199181967 START "/home/supadava/projects/astromech-wt-task-astromech-w62j/app"
1792199181968 COMPLETED 355 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
```

## Stderr

Bytes: 0
Truncated: no

```text
```

