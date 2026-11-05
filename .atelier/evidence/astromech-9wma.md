---
created_at: "2026-11-05T00:49:31.267340618+00:00"
id: "astromech-9wma"
evidence_type: "test"
commit: "928f402fa7872cad1b18e2f73d1a1943f2c7f018"
result: "pass"
captured_at: "2026-11-05T00:49:29.704979715+00:00"
command: "cargo test -q -p astromech-core --lib pipeline"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-b0tn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-b0tn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-core pipeline tests, including the just-left-journal test, on the branch merged with main"
updated_at: "2026-11-05T00:49:31.267600512+00:00"
---

## Summary

astromech-core pipeline tests, including the just-left-journal test, on the branch merged with main

## Command

```console
cargo test -q -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 183
SHA-256: 7d7a9665d053755995c8e6b2644f896a9da56dfea722deed545919a30af33a76
Test summary: test result: ok. 66 passed; 0 failed; 0 ignored; 0 measured; 138 filtered out; finished in 0.57s

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
