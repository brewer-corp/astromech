---
created_at: "2026-10-17T20:13:33.321911303+00:00"
id: "astromech-mn8a"
evidence_type: "test"
commit: "88b3f919b233288c847bedc451174d759478b8f9"
result: "pass"
captured_at: "2026-10-17T20:12:49.525485276+00:00"
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
title: "after review fixes: gear_down a_replayed_older_journal... passes 50/50 in a row under 24 CPU busy loops on 12 cores (fails 3/3 with the pre-fix pipeline.rs)"
updated_at: "2026-10-17T20:13:33.322209990+00:00"
---

## Summary

after review fixes: gear_down a_replayed_older_journal... passes 50/50 in a row under 24 CPU busy loops on 12 cores (fails 3/3 with the pre-fix pipeline.rs)

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

