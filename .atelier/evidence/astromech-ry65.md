---
created_at: "2026-10-17T00:44:51.478354823+00:00"
id: "astromech-ry65"
evidence_type: "test"
commit: "0ff3cb471b5605db3f0dd1757a362d6c9171108c"
result: "pass"
captured_at: "2026-10-17T00:44:50.894449405+00:00"
command: "cargo test -p astromech-colonisation"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-y9cj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-y9cj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Transfer/Died hold tests pass (failed before fix); fmt, clippy, workspace tests, check-deps, metadata green"
updated_at: "2026-10-17T00:44:51.478821915+00:00"
---

## Summary

Transfer/Died hold tests pass (failed before fix); fmt, clippy, workspace tests, check-deps, metadata green

## Command

```console
cargo test -p astromech-colonisation
```

Exit status: 0

## Stdout

Bytes: 1406
Truncated: no

```text

running 18 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::shopping_tests::carrier_transfers_move_cargo_on_and_off_the_ship ... ok
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::claims_and_beacons ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::shopping_tests::dying_clears_the_hold ... ok
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::shopping_tests::cargo_json_overrides_the_replayed_hold ... ok
test tests::save_writes_only_changes ... ok
test tests::survives_restart ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 335
Truncated: no

```text
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-y9cj/crates/astromech-colonisation)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-c0af0f73d6fbdc28)
   Doc-tests astromech_colonisation
```

