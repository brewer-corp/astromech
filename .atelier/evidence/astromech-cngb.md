---
created_at: "2026-10-17T20:18:53.151894960+00:00"
id: "astromech-cngb"
evidence_type: "test"
commit: "45c03711241845ad2fc529b3003117c73555fca9"
result: "pass"
captured_at: "2026-10-17T20:18:09.364420207+00:00"
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
title: "after merging origin/main: gear_down a_replayed_older_journal... 50/50 in a row under 24 CPU busy loops"
updated_at: "2026-10-17T20:18:53.152146770+00:00"
---

## Summary

after merging origin/main: gear_down a_replayed_older_journal... 50/50 in a row under 24 CPU busy loops

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

