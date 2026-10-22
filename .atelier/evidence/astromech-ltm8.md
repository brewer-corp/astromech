---
created_at: "2026-10-22T21:30:39.246055714+00:00"
id: "astromech-ltm8"
evidence_type: "test"
commit: "dfcb12fd136d61438891084e3e8672dc52cabd6c"
result: "pass"
captured_at: "2026-10-22T21:30:38.839192206+00:00"
command: "cargo test -p astromech-history --lib -- journals_from_before_fid two_fids continuation_part"
exit_status: "0"
agent_identity: "claude-implementer-fdh3"
target:
  kind: "issue"
  id: "astromech-yrfv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yrfv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Old LoadGame without FID plus later FID journal with the same name: one commander, old visits in its logbook, in either import order, after rebuild and after commanders are recomputed; two FIDs with one name stay apart; a continuation part of another session gets no commander"
updated_at: "2026-10-22T21:30:39.246544194+00:00"
---

## Summary

Old LoadGame without FID plus later FID journal with the same name: one commander, old visits in its logbook, in either import order, after rebuild and after commanders are recomputed; two FIDs with one name stay apart; a continuation part of another session gets no commander

## Command

```console
cargo test -p astromech-history --lib -- journals_from_before_fid two_fids continuation_part
```

Exit status: 0

## Stdout

Bytes: 375
Truncated: no

```text

running 4 tests
test tests::continuation_part_of_another_session_has_no_commander ... ok
test tests::two_fids_with_one_name_stay_apart ... ok
test tests::continuation_part_keeps_the_commander ... ok
test tests::journals_from_before_fid_join_the_commander_with_that_name ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.04s
```

## Stderr

Bytes: 162
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_history-8b4e2bbed412436b)
```

