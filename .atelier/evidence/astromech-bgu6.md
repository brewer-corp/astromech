---
created_at: "2026-11-04T20:51:02.396483904+00:00"
id: "astromech-bgu6"
evidence_type: "test"
commit: "12dfbfe736f1f7e0a7f91351ebd7d3e80c50df48"
result: "pass"
captured_at: "2026-11-04T20:50:43.589443588+00:00"
command: "cargo test -p astromech-core -p astromech-journal"
exit_status: "0"
agent_identity: "implement-agent-ki3j"
target:
  kind: "issue"
  id: "astromech-ki3j"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ki3j"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-core and astromech-journal suites pass at 12dfbfe7, including pipeline::tests::the_catch_up_follows_the_games_journal_not_a_touched_older_one (state after catch-up is the current journal's ship/system/credits, touched older journal never followed afterwards; both halves fail with the fix reverted) and the nrhv replay test the_games_journal_is_a_replay_up_to_startup_after_a_catch_up_on_a_touched_older_one"
updated_at: "2026-11-04T20:51:02.399776452+00:00"
---

## Summary

astromech-core and astromech-journal suites pass at 12dfbfe7, including pipeline::tests::the_catch_up_follows_the_games_journal_not_a_touched_older_one (state after catch-up is the current journal's ship/system/credits, touched older journal never followed afterwards; both halves fail with the fix reverted) and the nrhv replay test the_games_journal_is_a_replay_up_to_startup_after_a_catch_up_on_a_touched_older_one

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 23470
SHA-256: e4bee0627fe2306b0cd7d4c8cf3a39ace061af686b2fa72918a361c9c06719ca
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 2410
SHA-256: d034efea06edc7e7beb1a5e6cc7b838f80000de22626aef5f7ec42f25ad44438
