---
created_at: "2026-11-04T23:03:40.205710786+00:00"
id: "astromech-vwue"
evidence_type: "test"
commit: "a184890993d41521fecbde50296234620a986f08"
result: "pass"
captured_at: "2026-11-04T23:02:47.735864178+00:00"
command: "cargo test -p astromech-store -p astromech-core"
exit_status: "0"
agent_identity: "implement-agent-zirr"
target:
  kind: "issue"
  id: "astromech-zirr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zirr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-store -p astromech-core passes, including new paths::tests::a_note_left_by_a_core_that_is_gone_is_ignored (killed pid, reused pid under another exe name, old path-only note, all naming a deleted --config file; plus live note). That test failed with the liveness filter removed (deleted.toml returned instead of the default). fmt, clippy -D warnings (store, core) and check-deps.py clean."
updated_at: "2026-11-04T23:03:40.205989817+00:00"
---

## Summary

cargo test -p astromech-store -p astromech-core passes, including new paths::tests::a_note_left_by_a_core_that_is_gone_is_ignored (killed pid, reused pid under another exe name, old path-only note, all naming a deleted --config file; plus live note). That test failed with the liveness filter removed (deleted.toml returned instead of the default). fmt, clippy -D warnings (store, core) and check-deps.py clean.

## Command

```console
cargo test -p astromech-store -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 23234
SHA-256: f4986d276b37ff6e005ccb0279617e0451394f0a7357f6eb700c4c46e65f9686
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 3169
SHA-256: 9676f33807bec64a5cdbaf8823286be4069894c9388d91b389fb2126058de31d
