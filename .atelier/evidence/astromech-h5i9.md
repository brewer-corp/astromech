---
created_at: "2026-11-05T01:04:13.851195919+00:00"
id: "astromech-h5i9"
evidence_type: "test"
commit: "cc4fb5cadab7516d7ccb2041c8247f525d7181d5"
result: "pass"
captured_at: "2026-11-05T01:04:10.927031959+00:00"
command: "sh -c 'cargo test -q -p astromech-intent && cargo fmt --all -- --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-e0im"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e0im"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "intent tests and fmt on the branch merged with main (after 3n7q)"
updated_at: "2026-11-05T01:04:13.851528880+00:00"
---

## Summary

intent tests and fmt on the branch merged with main (after 3n7q)

## Command

```console
sh -c 'cargo test -q -p astromech-intent && cargo fmt --all -- --check'
```

Exit status: 0

## Stdout

Bytes: 265
SHA-256: 52a4b8c5f38a5c2367a420aed18a73654a1dc6e3850aae277cad79c010f78c5e
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
