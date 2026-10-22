---
created_at: "2026-10-22T21:25:02.934850447+00:00"
id: "astromech-kbw6"
evidence_type: "test"
commit: "8c11bfef29e9de07d0b94b7a7576847ad3111b24"
result: "pass"
captured_at: "2026-10-22T21:25:00.327234802+00:00"
command: "pnpm -C app exec vitest run src/modules/route"
exit_status: "0"
agent_identity: "claude-implementer-jo3l"
target:
  kind: "issue"
  id: "astromech-83og"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-83og"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Plot.test: the range line follows a cargo change within a 5 s poll and after a plot; the Following line ends with the plotted range"
updated_at: "2026-10-22T21:25:02.935305405+00:00"
---

## Summary

Plot.test: the range line follows a cargo change within a 5 s poll and after a plot; the Following line ends with the plotted range

## Command

```console
pnpm -C app exec vitest run src/modules/route
```

Exit status: 0

## Stdout

Bytes: 243
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt/astromech-jo3l/app


 Test Files  2 passed (2)
      Tests  5 passed (5)
   Start at  17:25:01
   Duration  1.63s (environment 46%, tests 25%, setup 14%, transform 13%, import 1%, worker 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

