---
created_at: "2026-10-16T20:46:02.463144924+00:00"
id: "astromech-59nr"
evidence_type: "test"
commit: "5ec872891199c78c841ce74dc64ee7bb15cfa311"
result: "pass"
captured_at: "2026-10-16T20:45:58.997329756+00:00"
command: "pnpm vitest run src/modules/datasharing"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-09s3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-09s3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 5ec8728, app: Settings → Data sharing tests (switch saved both ways; status row shows the last error 'HTTP 503 ...' from a failed send, last sent, queue depth; plain 'what gets sent' text)"
updated_at: "2026-10-16T20:46:02.463549019+00:00"
---

## Summary

At 5ec8728, app: Settings → Data sharing tests (switch saved both ways; status row shows the last error 'HTTP 503 ...' from a failed send, last sent, queue depth; plain 'what gets sent' text)

## Command

```console
pnpm vitest run src/modules/datasharing
```

Exit status: 0

## Stdout

Bytes: 240
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-tyjz/app


 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  16:46:00
   Duration  2.06s (environment 51%, tests 21%, setup 14%, transform 13%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

