---
created_at: "2026-10-16T19:28:57.690701511+00:00"
id: "astromech-dmsh"
evidence_type: "test"
commit: "97fc8152b0b9361747c51b68a28c12098a218b18"
result: "pass"
captured_at: "2026-10-16T19:28:48.432074807+00:00"
command: "bash -c 'cd app && pnpm test' -q"
exit_status: "0"
agent_identity: "orchestrator (Claude)"
target:
  kind: "issue"
  id: "astromech-kbcm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kbcm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Keybinds component tests pass at the branch head after the main merge (pnpm test, whole app suite)"
updated_at: "2026-10-16T19:28:57.693927538+00:00"
---

## Summary

Keybinds component tests pass at the branch head after the main merge (pnpm test, whole app suite)

## Command

```console
bash -c 'cd app && pnpm test' -q
```

Exit status: 0

## Stdout

Bytes: 335
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-q318/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-q318/app


 Test Files  4 passed (4)
      Tests  16 passed (16)
   Start at  15:28:49
   Duration  7.90s (environment 45%, tests 37%, transform 9%, setup 8%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

