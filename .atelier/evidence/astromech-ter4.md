---
created_at: "2026-10-16T23:03:40.820460943+00:00"
id: "astromech-ter4"
evidence_type: "test"
commit: "2e9bc9387a1f0325594aeb596114eb021636e7e8"
result: "pass"
captured_at: "2026-10-16T23:03:37.312436616+00:00"
command: "bash -c 'cd app && pnpm vitest run src/modules/colonisation src/lib/colonisation.test.ts src/lib/ui'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-05bi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-05bi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "UI component tests: Colonisation page lists what is left to buy in loads, asks for it from the site's system, shows Efremov Plant (3 of 4) first and FCX-801 (1 of 4) last with stock and pad; clearing the origin re-asks near the commander; unavailable warning; finished project not searched; empty and error states. Removing the site origin fails 2 tests (mutation checked)."
updated_at: "2026-10-16T23:03:40.820708334+00:00"
---

## Summary

UI component tests: Colonisation page lists what is left to buy in loads, asks for it from the site's system, shows Efremov Plant (3 of 4) first and FCX-801 (1 of 4) last with stock and pad; clearing the origin re-asks near the commander; unavailable warning; finished project not searched; empty and error states. Removing the site origin fails 2 tests (mutation checked).

## Command

```console
bash -c 'cd app && pnpm vitest run src/modules/colonisation src/lib/colonisation.test.ts src/lib/ui'
```

Exit status: 0

## Stdout

Bytes: 238
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-k3wp/app


 Test Files  4 passed (4)
      Tests  9 passed (9)
   Start at  19:03:38
   Duration  2.04s (environment 69%, setup 11%, tests 9%, transform 9%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

