---
created_at: "2026-11-05T16:01:35.412294798+00:00"
id: "astromech-o22a"
evidence_type: "test"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:00:50.712513892+00:00"
command: "cargo test -p astromech-core -p astromech-galaxy -p astromech-market -p astromech-tools"
exit_status: "0"
agent_identity: "validator-fable-kyqw"
target:
  kind: "issue"
  id: "astromech-kyqw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kyqw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Rust tests for the galaxy rework crates (astromech-core, astromech-galaxy, astromech-market, astromech-tools) run fresh on feature/astromech-51ue; the earlier full log showed 507 passed, 0 failed, 2 ignored (one needs the voice worker and model, one the real neutron star file)"
updated_at: "2026-11-05T16:01:35.412572476+00:00"
---

## Summary

Rust tests for the galaxy rework crates (astromech-core, astromech-galaxy, astromech-market, astromech-tools) run fresh on feature/astromech-51ue; the earlier full log showed 507 passed, 0 failed, 2 ignored (one needs the voice worker and model, one the real neutron star file)

## Command

```console
cargo test -p astromech-core -p astromech-galaxy -p astromech-market -p astromech-tools
```

Exit status: 0

## Stdout

Bytes: 38603
SHA-256: e5e28a1168d8a633448c6f8ff0351627df06afa03907b3f894b8cde9bbde9ecf
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 4313
SHA-256: 248ab4aadd93b169c8430afcecf7f326b6e016ded46851e5f0f8c013d637006e
