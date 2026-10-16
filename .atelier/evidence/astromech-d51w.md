---
created_at: "2026-10-16T19:37:26.363111859+00:00"
id: "astromech-d51w"
evidence_type: "test"
commit: "44d34e14e21782f231c779bbe9affb94f543d4a9"
result: "pass"
captured_at: "2026-10-16T19:37:24.934651974+00:00"
command: "cargo test -p astromech-colonisation -p astromech-store"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6qk1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6qk1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Replay of a synthetic depot/contribution/claim/beacon journal gives expected progress per project; state restored after reopening the SQLite store (8 colonisation tests, 1 store test)"
updated_at: "2026-10-16T19:37:26.363518437+00:00"
---

## Summary

Replay of a synthetic depot/contribution/claim/beacon journal gives expected progress per project; state restored after reopening the SQLite store (8 colonisation tests, 1 store test)

## Command

```console
cargo test -p astromech-colonisation -p astromech-store
```

Exit status: 0

## Stdout

Bytes: 1010
Truncated: no

```text

running 8 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::claims_and_beacons ... ok
test tests::survives_restart ... ok
test tests::save_writes_only_changes ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 3 tests
test tests::records_upsert_and_list_by_kind ... ok
test tests::migrates_and_stores ... ok
test tests::learns_aliases_and_caps ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 454
Truncated: no

```text
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-k3wp/crates/astromech-colonisation)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.99s
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-7ff6cab3cf8b5770)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-ec52dbef3d32c891)
   Doc-tests astromech_colonisation
   Doc-tests astromech_store
```

