---
created_at: "2026-10-22T23:43:07.074791938+00:00"
id: "astromech-0j4u"
evidence_type: "validation"
commit: "dc7ecd121264b4d305a593a1f947f0f9b0d0c79c"
result: "pass"
captured_at: "2026-10-22T23:40:57.058222213+00:00"
command: "bash -c 'python3 -c \"import yaml,sys; yaml.safe_load(open(sys.argv[1]))\" .github/workflows/atelier.yml && echo yaml ok && S=/tmp/tmp.MHkkfREwMf/ci.sh U=file://$PWD && $S $U HEAD && $S $U e5fe2f51 && ! $S $U fa3b668e'"
exit_status: "0"
agent_identity: "claude-implementer-mk30"
target:
  kind: "issue"
  id: "astromech-mk30"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mk30"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workflow YAML parses; its doctor step (main at tip, detached empty commit on top) simulated in fresh clones with the pinned build passes on this branch head and on e5fe2f51 (tip adds an approval), and fails on fa3b668e (y296 approval missing) with the expected findings.disposed error"
updated_at: "2026-10-22T23:43:07.075066149+00:00"
---

## Summary

Workflow YAML parses; its doctor step (main at tip, detached empty commit on top) simulated in fresh clones with the pinned build passes on this branch head and on e5fe2f51 (tip adds an approval), and fails on fa3b668e (y296 approval missing) with the expected findings.disposed error

## Command

```console
bash -c 'python3 -c "import yaml,sys; yaml.safe_load(open(sys.argv[1]))" .github/workflows/atelier.yml && echo yaml ok && S=/tmp/tmp.MHkkfREwMf/ci.sh U=file://$PWD && $S $U HEAD && $S $U e5fe2f51 && ! $S $U fa3b668e'
```

Exit status: 0

## Stdout

Bytes: 470
Truncated: no

```text
yaml ok
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
No work owner on detached HEAD; it changes no Atelier tracker files since main, so there is nothing to land.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
```

## Stderr

Bytes: 515
Truncated: no

```text
HEAD is now at dc7ecd12 Run atelier doctor in CI (astromech-mk30)
HEAD is now at e5fe2f51 atelier: approval of astromech-y296 at its closing commit
HEAD is now at fa3b668e atelier: file astromech-6h7t (Spansh neutron jumps)
Error: land_invariants_unmet:
session findings.disposed: astromech-h9kb: unresolved FIX NOW: resolution needs fresh independent approval; astromech-zdy2: unresolved FIX NOW: resolution needs fresh independent approval
Resolve the named checks, commit the records, then rerun `atelier land`.
```

