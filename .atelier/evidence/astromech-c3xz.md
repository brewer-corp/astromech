---
created_at: "2026-10-16T19:29:04.645128634+00:00"
id: "astromech-c3xz"
evidence_type: "test"
commit: "97fc8152b0b9361747c51b68a28c12098a218b18"
result: "pass"
captured_at: "2026-10-16T19:28:58.079916338+00:00"
command: "bash -c 'cd app && pnpm test' -q"
exit_status: "0"
agent_identity: "orchestrator (Claude)"
target:
  kind: "issue"
  id: "astromech-rhfn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-rhfn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Keybinds component tests pass at the branch head after the main merge (pnpm test, whole app suite)"
updated_at: "2026-10-16T19:29:04.645377128+00:00"
---

## Summary

Keybinds component tests pass at the branch head after the main merge (pnpm test, whole app suite)

## Command

```console
bash -c 'cd app && pnpm test' -q
```

Exit status: 0

## Stdout

Bytes: 336
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-q318/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-q318/app


 Test Files  4 passed (4)
      Tests  16 passed (16)
   Start at  15:28:59
   Duration  5.56s (tests 61%, environment 22%, transform 11%, setup 5%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

