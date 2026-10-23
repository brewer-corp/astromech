---
created_at: "2026-10-23T00:50:24.960598208+00:00"
id: "astromech-axzf"
evidence_type: "test"
commit: "9f376ee030beefc92ba87f5f0d99b4ce03c7cc78"
result: "pass"
captured_at: "2026-10-23T00:50:24.649857502+00:00"
command: "env CARGO_BUILD_JOBS=4 cargo build --workspace --locked"
exit_status: "0"
agent_identity: "claude-implementer-rcui"
target:
  kind: "issue"
  id: "astromech-rcui"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-rcui"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace builds at 1.0.0-alpha.3 with the committed Cargo.lock (--locked)"
updated_at: "2026-10-23T00:50:24.960870064+00:00"
---

## Summary

Workspace builds at 1.0.0-alpha.3 with the committed Cargo.lock (--locked)

## Command

```console
env CARGO_BUILD_JOBS=4 cargo build --workspace --locked
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 72
Truncated: no

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

