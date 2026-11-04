---
created_at: "2026-11-04T21:43:11.310054238+00:00"
id: "astromech-1wwg"
evidence_type: "test"
commit: "b5404b19e98c7dfbcd1554df73452b552c8efc7d"
result: "pass"
captured_at: "2026-11-04T21:42:53.576284488+00:00"
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
title: "astromech-core and astromech-journal suites pass at b5404b19, including a_line_the_game_writes_to_its_journal_during_the_catch_up_is_live and a_journal_the_game_starts_during_the_catch_up_is_live (both fail before the fix: 0 sent instead of 1 and 2; history in each journal is not sent) and the_followed_journal_past_its_startup_length_is_live_after_the_catch_up (sent and spoken once, after the catch-up)"
updated_at: "2026-11-04T21:43:11.310406857+00:00"
---

## Summary

astromech-core and astromech-journal suites pass at b5404b19, including a_line_the_game_writes_to_its_journal_during_the_catch_up_is_live and a_journal_the_game_starts_during_the_catch_up_is_live (both fail before the fix: 0 sent instead of 1 and 2; history in each journal is not sent) and the_followed_journal_past_its_startup_length_is_live_after_the_catch_up (sent and spoken once, after the catch-up)

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 24137
SHA-256: be29d8bb88c28c7f31519bc1acca409b9722bf70acc6000df796c515a1a9559c
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 2410
SHA-256: 606272b56bfbe8ab77aa17945bb1ff90f68062761351ea279f63fe2b85aab36d
