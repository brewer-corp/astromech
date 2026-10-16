---
created_at: "2026-10-16T19:55:16.335547286+00:00"
id: "astromech-iisp"
evidence_type: "test"
commit: "49fd4da658c9b34062477dae76127643b814b925"
result: "pass"
captured_at: "2026-10-16T19:55:13.829547863+00:00"
command: "pnpm --dir app exec vitest run src/modules/settings"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-orel"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-orel"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Settings → Voice responses UI against the recorded core: British-first list, saves speaker/colour/speed, preview uses the chosen voice, preview refused without models"
updated_at: "2026-10-16T19:55:16.335970075+00:00"
---

## Summary

Settings → Voice responses UI against the recorded core: British-first list, saves speaker/colour/speed, preview uses the chosen voice, preview refused without models

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
      Tests  7 passed (7)
   Start at  15:55:14
   Duration  1.67s (tests 59%, environment 27%, setup 7%, transform 6%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

