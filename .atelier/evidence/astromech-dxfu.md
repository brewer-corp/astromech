---
created_at: "2026-10-22T23:40:56.903500111+00:00"
id: "astromech-dxfu"
evidence_type: "test"
commit: "dc7ecd121264b4d305a593a1f947f0f9b0d0c79c"
result: "pass"
captured_at: "2026-10-22T23:39:59.089909769+00:00"
command: "/tmp/tmp.xJQQSIGjtj/bin/atelier doctor"
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
title: "Pinned Atelier (spadaval/atelier 1f5fd1c2, built with cargo install --locked into a scratch root) passes atelier doctor on this branch: every done claim and the whole review history checked"
updated_at: "2026-10-22T23:40:56.903785873+00:00"
---

## Summary

Pinned Atelier (spadaval/atelier 1f5fd1c2, built with cargo install --locked into a scratch root) passes atelier doctor on this branch: every done claim and the whole review history checked

## Command

```console
/tmp/tmp.xJQQSIGjtj/bin/atelier doctor
```

Exit status: 0

## Stdout

Bytes: 392
Truncated: no

```text
Base main has 21 commits this branch does not. Merge them in with `git merge main` and record fresh proof before atelier merge; a clean merge keeps an approval.
Owner: astromech-mk30 in_progress. In-progress work can hand off; completed claims satisfy applicable gates.
Landing checks passed. No commits or records created.
Doctor: every done claim and the whole review history were checked.
```

## Stderr

Bytes: 0
Truncated: no

```text
```

