---
created_at: "2026-10-17T00:03:39.162232680+00:00"
id: "astromech-9m4k"
evidence_type: "test"
commit: "ea70d32458c19a631a8cc5aa70392e850ed30573"
result: "pass"
captured_at: "2026-10-17T00:03:38.050926239+00:00"
command: "cargo test -p astromech-core --lib -- history a_journal_folder"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-5t34"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-5t34"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "set_folders after startup imports the new journal folder; re-pointing waits for the old follower to end and stops following the old folder (failed before the fix / with the join removed)"
updated_at: "2026-10-17T00:03:39.162484560+00:00"
---

## Summary

set_folders after startup imports the new journal folder; re-pointing waits for the old follower to end and stops following the old folder (failed before the fix / with the join removed)

## Command

```console
cargo test -p astromech-core --lib -- history a_journal_folder
```

Exit status: 0

## Stdout

Bytes: 461
Truncated: no

```text

running 5 tests
test app::tests::a_journal_folder_set_after_startup_is_imported ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test history::tests::re_pointing_stops_the_old_follower_first ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 68 filtered out; finished in 0.90s
```

## Stderr

Bytes: 196
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_core-094d1cbbf733166e)
```

