---
created_at: "2026-10-17T00:27:46.914976485+00:00"
id: "astromech-rjdw"
evidence_type: "test"
commit: "68d91e885079728ef2b8dc52c70fecf15cdb79af"
result: "pass"
captured_at: "2026-10-17T00:27:43.148225435+00:00"
command: "bash -c 'cd app && pnpm vitest run src/modules/trade src/lib/trade.test.ts'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-d1lo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-d1lo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Component and lib tests: Trade view switches Sell to Anywhere (calls tradeAnywhere, title 'Best prices anywhere', radius filter hidden, prices descending from 390,780 with 'reported' age), Buy anywhere, back to Near (tradeSearch); tradeAnywhere preview ignores the radius and drops carriers on request."
updated_at: "2026-10-17T00:27:46.915290942+00:00"
---

## Summary

Component and lib tests: Trade view switches Sell to Anywhere (calls tradeAnywhere, title 'Best prices anywhere', radius filter hidden, prices descending from 390,780 with 'reported' age), Buy anywhere, back to Near (tradeSearch); tradeAnywhere preview ignores the radius and drops carriers on request.

## Command

```console
bash -c 'cd app && pnpm vitest run src/modules/trade src/lib/trade.test.ts'
```

Exit status: 0

## Stdout

Bytes: 236
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-d1lo/app


 Test Files  2 passed (2)
      Tests  5 passed (5)
   Start at  20:27:44
   Duration  1.91s (environment 69%, setup 11%, tests 10%, transform 9%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

