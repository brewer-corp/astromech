---
created_at: "2026-11-04T22:57:01.985209503+00:00"
id: "astromech-isib"
evidence_type: "test"
commit: "36f1203fed380522464400dae7e384a5756fc4f3"
result: "pass"
captured_at: "2026-11-04T22:56:38.040007351+00:00"
command: "cargo test -p astromech-core -p astromech-journal"
exit_status: "0"
agent_identity: "implement-agent-ki3j"
target:
  kind: "issue"
  id: "astromech-zftk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zftk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-core and astromech-journal suites pass at 36f1203f, including a_followed_journal_that_cannot_be_measured_is_replayed_in_full (6rig; fails with the listing fallback restored), a_followed_journal_unreadable_through_the_catch_up_is_history_once_read and a_followed_journal_unreadable_for_the_replay_is_split_at_its_startup_length (p58u; both fail without open()'s early return), and the earlier zftk tests"
updated_at: "2026-11-04T22:57:01.988958418+00:00"
---

## Summary

astromech-core and astromech-journal suites pass at 36f1203f, including a_followed_journal_that_cannot_be_measured_is_replayed_in_full (6rig; fails with the listing fallback restored), a_followed_journal_unreadable_through_the_catch_up_is_history_once_read and a_followed_journal_unreadable_for_the_replay_is_split_at_its_startup_length (p58u; both fail without open()'s early return), and the earlier zftk tests

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 24435
SHA-256: e2766abd02929d1ae6db99f69331428255342567cd4c0d6b74c76dabdb0b45f4
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 4323
SHA-256: 222134c6c35abce5b0b4129ee66b9de5b773df897078625adbf651a8d2acb619
