---
created_at: "2026-10-17T20:06:13.416477501+00:00"
id: "astromech-zn1z"
evidence_type: "test"
commit: "33bdbfda02fb10964e7435021df7a1c5519ed0f6"
result: "pass"
captured_at: "2026-10-17T20:05:30.813924832+00:00"
command: "bash target/tmp/loop50.sh"
exit_status: "0"
agent_identity: "claude-diagnose"
target:
  kind: "issue"
  id: "astromech-0d7l"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-0d7l"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "gear_down a_replayed_older_journal... (now forcing the create-then-write interleaving) passes 50/50 in a row under 24 CPU busy loops on 12 cores; it failed 5/5 on the unfixed pipeline"
updated_at: "2026-10-17T20:06:13.416774095+00:00"
---

## Summary

gear_down a_replayed_older_journal... (now forcing the create-then-write interleaving) passes 50/50 in a row under 24 CPU busy loops on 12 cores; it failed 5/5 on the unfixed pipeline

## Command

```console
bash target/tmp/loop50.sh
```

Exit status: 0

## Stdout

Bytes: 24
Truncated: no

```text
50/50 passed under load
```

## Stderr

Bytes: 0
Truncated: no

```text
```

