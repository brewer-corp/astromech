---
created_at: "2026-11-04T23:35:00.472767907+00:00"
id: "astromech-68x5"
evidence_type: "test"
commit: "df0c707bb2f7e8355b0c2ad3b9e28f6d3675b111"
result: "pass"
captured_at: "2026-11-04T23:34:17.747740520+00:00"
command: "cargo test -p astromech-core -p astromech-journal"
exit_status: "0"
agent_identity: "implement-agent-ki3j"
target:
  kind: "issue"
  id: "astromech-rjhy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-rjhy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-core and astromech-journal suites pass, including the_games_journal_beside_a_later_foreign_one_is_measured_by_handle (the game's journal, neither latest nor followed, beside a later-stamped foreign one, is measured by handle and a 3-day-old one is not; lengths_by_handle 2, fails with the recent rule disabled: 1; the game's next line is the only one sent)"
updated_at: "2026-11-04T23:35:00.473049433+00:00"
---

## Summary

astromech-core and astromech-journal suites pass, including the_games_journal_beside_a_later_foreign_one_is_measured_by_handle (the game's journal, neither latest nor followed, beside a later-stamped foreign one, is measured by handle and a 3-day-old one is not; lengths_by_handle 2, fails with the recent rule disabled: 1; the game's next line is the only one sent)

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 25535
SHA-256: a38b34a66466bc052cfa9a4fce6200ce8f1d2af379c7fa597550023b44d0915b
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 4535
SHA-256: 1fcc9a2458a372e2fa67014f9eacca9f796229e647113c580f8eadcd58cbd049
