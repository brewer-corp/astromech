---
created_at: "2026-11-04T20:39:01.111231832+00:00"
id: "astromech-olxq"
evidence_type: "test"
commit: "7fb74f89f206278cf9f73034c33d12c4152ba314"
result: "pass"
captured_at: "2026-11-04T20:39:00.818287181+00:00"
command: "cargo test -p astromech-engineering --lib links"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-fyvh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-fyvh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "links tests: dot segments (.., ., %2e%2e, %2E%2E, .%2e, %2e., %2e) and backslashes refused under every allowed prefix; every link shape the app builds (Coriolis, EDSY, Spansh, Inara, GitHub guide/releases) still passes"
updated_at: "2026-11-04T20:39:01.111580532+00:00"
---

## Summary

links tests: dot segments (.., ., %2e%2e, %2E%2E, .%2e, %2e., %2e) and backslashes refused under every allowed prefix; every link shape the app builds (Coriolis, EDSY, Spansh, Inara, GitHub guide/releases) still passes

## Command

```console
cargo test -p astromech-engineering --lib links
```

Exit status: 0

## Stdout

Bytes: 525
SHA-256: d1fec4e8134f89fdd2a05b166a98ee6cc7377c414e126538a6175255d71c9ff2
Test summary: test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.02s

## Stderr

Bytes: 200
SHA-256: ddec908c5e17ed6819cfe9a1b7db90f8c43ec3ab49cccef6a56d7c4000234c5d
