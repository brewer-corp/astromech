---
created_at: "2026-10-17T19:28:58.546431190+00:00"
id: "astromech-ls4n"
evidence_type: "test"
commit: "ba4b38118cd7362d01c935da8a1877bb4653668e"
result: "pass"
captured_at: "2026-10-17T19:28:58.379677228+00:00"
command: "python3 scripts/check-guide.py"
exit_status: "0"
agent_identity: "claude-author (Opus)"
target:
  kind: "issue"
  id: "astromech-dpoc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-dpoc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Guide link and heading check at HEAD (docs/guide routes.md, voice-commands.md, callouts.md, index.md, galaxy.md). Accuracy against the code: independent docs review rounds 1-2 (review-agent, sonnet) recorded on astromech-dpoc."
updated_at: "2026-10-17T19:28:58.546733854+00:00"
---

## Summary

Guide link and heading check at HEAD (docs/guide routes.md, voice-commands.md, callouts.md, index.md, galaxy.md). Accuracy against the code: independent docs review rounds 1-2 (review-agent, sonnet) recorded on astromech-dpoc.

## Command

```console
python3 scripts/check-guide.py
```

Exit status: 0

## Stdout

Bytes: 37
Truncated: no

```text
checked 22 guide pages: 0 problem(s)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

