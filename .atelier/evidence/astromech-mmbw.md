---
created_at: "2026-11-04T23:23:01.835860970+00:00"
id: "astromech-mmbw"
evidence_type: "test"
commit: "cdf3f2490fcc0ff4b51cd87757002f8c517710ef"
result: "pass"
captured_at: "2026-11-04T23:22:27.164627563+00:00"
command: "cargo test -p astromech-core -p astromech-journal"
exit_status: "0"
agent_identity: "implement-agent-ki3j"
target:
  kind: "issue"
  id: "astromech-73w9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-73w9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Rerun after a shared-cache collision (astromech-3np4 failed on stale artifacts from another worktree, not on this code): astromech-core and astromech-journal suites pass at 6fda4a5f, including a_refused_touched_journals_header_is_read_once (fails with the cache disabled: 20 reads vs 5) and its rename-replacement step"
updated_at: "2026-11-04T23:23:01.836375831+00:00"
---

## Summary

Rerun after a shared-cache collision (astromech-3np4 failed on stale artifacts from another worktree, not on this code): astromech-core and astromech-journal suites pass at 6fda4a5f, including a_refused_touched_journals_header_is_read_once (fails with the cache disabled: 20 reads vs 5) and its rename-replacement step

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 25130
SHA-256: 3336ae64907fee3c0114c97d7b8e88366dc1d03842f355a93e2ce218c0371b97
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 2410
SHA-256: 7424ef6da08451c44402a4c8aac23c7f97632022200ad1b7833ff930a7be326c
