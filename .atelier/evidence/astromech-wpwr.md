---
created_at: "2026-10-16T19:17:04.470519470+00:00"
id: "astromech-wpwr"
evidence_type: "test"
commit: "4a43d0804ae0e8b29ee9beba1cf1d4ef0ba0909c"
result: "pass"
captured_at: "2026-10-16T19:16:57.285847731+00:00"
command: "pnpm --dir app exec vitest run src/modules/settings"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ny3y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ny3y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Settings → Microphone shows the stored gate and calibrates through the dialog (vitest, recorded core)"
updated_at: "2026-10-16T19:17:04.470800374+00:00"
---

## Summary

Settings → Microphone shows the stored gate and calibrates through the dialog (vitest, recorded core)

## Command

```console
pnpm --dir app exec vitest run src/modules/settings
```

Exit status: 0

## Stdout

Bytes: 238
Truncated: no

```text

 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-lxma/app


 Test Files  1 passed (1)
      Tests  4 passed (4)
   Start at  15:17:02
   Duration  1.73s (tests 52%, environment 30%, transform 8%, setup 8%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

