---
created_at: "2026-10-16T19:45:22.135942312+00:00"
id: "astromech-ndxe"
evidence_type: "test"
commit: "aa4606da45d3ef2a0bd7dbbc81a309666ea2a45c"
result: "pass"
captured_at: "2026-10-16T19:45:21.853006322+00:00"
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
title: "Shopping list review fixes: hold tracks buys, sells, collects, ejects and contributions; 14 colonisation tests pass"
updated_at: "2026-10-16T19:45:22.136373748+00:00"
---

## Summary

Shopping list review fixes: hold tracks buys, sells, collects, ejects and contributions; 14 colonisation tests pass

## Command

```console
cargo test -p astromech-colonisation
```

Exit status: 0

## Stdout

Bytes: 1095
Truncated: no

```text

running 14 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::claims_and_beacons ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::save_writes_only_changes ... ok
test tests::survives_restart ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 203
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-7ff6cab3cf8b5770)
   Doc-tests astromech_colonisation
```

