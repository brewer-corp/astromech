---
created_at: "2026-11-05T00:54:08.604088189+00:00"
id: "astromech-wetd"
evidence_type: "test"
commit: "50caf193bd24d739b2bfa576043bbfaf85aa2244"
result: "pass"
captured_at: "2026-11-05T00:53:27.742640529+00:00"
command: "cargo test -q -p astromech-intent -p astromech-voice -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3n7q"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3n7q"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "intent, voice and core tests on the branch merged with main (after b0tn)"
updated_at: "2026-11-05T00:54:08.604348053+00:00"
---

## Summary

intent, voice and core tests on the branch merged with main (after b0tn)

## Command

```console
cargo test -q -p astromech-intent -p astromech-voice -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 2770
SHA-256: 448291f9e5da7ee622f4940f33ba2340fc87aa81740d6de3e2fa92e468826985
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
