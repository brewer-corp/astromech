---
created_at: "2026-11-05T01:08:49.798264261+00:00"
id: "astromech-1ddp"
evidence_type: "test"
commit: "26498ab1e98748da405b617be5f12ceeb20702e3"
result: "pass"
captured_at: "2026-11-05T01:08:04.166071766+00:00"
command: "sh -c 'cargo test -q -p astromech-announce -p astromech-core -j 4 && cargo clippy -q -p astromech-announce --all-targets -j 4 -- -D warnings && cargo fmt --all -- --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-k1fw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-k1fw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "announce and core tests, clippy and fmt on the branch merged with main"
updated_at: "2026-11-05T01:08:49.798928982+00:00"
---

## Summary

announce and core tests, clippy and fmt on the branch merged with main

## Command

```console
sh -c 'cargo test -q -p astromech-announce -p astromech-core -j 4 && cargo clippy -q -p astromech-announce --all-targets -j 4 -- -D warnings && cargo fmt --all -- --check'
```

Exit status: 0

## Stdout

Bytes: 4539
SHA-256: 4c06b6832d665f51435743ba53b97db1d4b97807cbc5e36c473f3dea84134b1a
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
