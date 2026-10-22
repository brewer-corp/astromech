---
created_at: "2026-10-22T20:56:02.763382880+00:00"
id: "astromech-avvj"
evidence_type: "test"
commit: "81cf4fbbd859a5eadef2d3cb1d54754fe08bbe6c"
result: "pass"
captured_at: "2026-10-22T20:56:01.881658427+00:00"
command: "cargo deny check advisories"
exit_status: "0"
agent_identity: "orchestrator-claude"
target:
  kind: "issue"
  id: "astromech-84vh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-84vh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo deny check advisories passes with the bumped lockfile"
updated_at: "2026-10-22T20:56:02.763694912+00:00"
---

## Summary

cargo deny check advisories passes with the bumped lockfile

## Command

```console
cargo deny check advisories
```

Exit status: 0

## Stdout

Bytes: 14
Truncated: no

```text
advisories ok
```

## Stderr

Bytes: 0
Truncated: no

```text
```

