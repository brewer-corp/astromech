---
created_at: "2026-11-04T21:05:44.548228289+00:00"
id: "astromech-63mv"
evidence_type: "test"
commit: "655647a5b127e6319f3d5787405a6b284d105301"
result: "pass"
captured_at: "2026-11-04T21:05:39.162257467+00:00"
command: "cargo test -p astromech-data --lib download"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-ksb3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ksb3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "a_refused_connection_keeps_its_reason: a refused local connection's DownloadError carries 'Connection refused (os error 111)' in detail() while Display stays the plain sentence. The shell's log line is not compiled on Linux (no webkit2gtk); Windows CI builds it."
updated_at: "2026-11-04T21:05:44.548729073+00:00"
---

## Summary

a_refused_connection_keeps_its_reason: a refused local connection's DownloadError carries 'Connection refused (os error 111)' in detail() while Display stays the plain sentence. The shell's log line is not compiled on Linux (no webkit2gtk); Windows CI builds it.

## Command

```console
cargo test -p astromech-data --lib download
```

Exit status: 0

## Stdout

Bytes: 179
SHA-256: 0f0663b57f0e7df4126da68d5bc21f9f741935d8acc0f6b071e6c0dbaab5718f
Test summary: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

## Stderr

Bytes: 352
SHA-256: 5e601a16b100442f9f2b895a9d02c8abeb686e7694a1051fda18a8198a96ce45
