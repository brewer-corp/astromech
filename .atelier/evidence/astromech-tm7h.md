---
created_at: "2026-10-16T19:42:38.940987797+00:00"
id: "astromech-tm7h"
evidence_type: "test"
commit: "95cfe13bacbae9f4161f5832ac732b41f5f9d490"
result: "pass"
captured_at: "2026-10-16T19:42:38.757432729+00:00"
command: "cargo test -p astromech-colonisation"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-i9w7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-i9w7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fixture project, cargo and ship give the right remaining list and load count (3 shopping tests; 11 colonisation tests)"
updated_at: "2026-10-16T19:42:38.941257651+00:00"
---

## Summary

Fixture project, cargo and ship give the right remaining list and load count (3 shopping tests; 11 colonisation tests)

## Command

```console
cargo test -p astromech-colonisation
```

Exit status: 0

## Stdout

Bytes: 877
Truncated: no

```text

running 11 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::shopping_tests::names_match_between_depot_and_cargo ... ok
test tests::claims_and_beacons ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::save_writes_only_changes ... ok
test tests::survives_restart ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 203
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-7ff6cab3cf8b5770)
   Doc-tests astromech_colonisation
```

