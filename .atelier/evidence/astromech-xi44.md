---
created_at: "2026-10-16T18:59:18.150928110+00:00"
id: "astromech-xi44"
evidence_type: "test"
commit: "d3e0f9b27b46340126ef1ac3dcd97656c1a57604"
result: "pass"
captured_at: "2026-10-16T18:59:17.918069591+00:00"
command: "cargo test -p astromech-engineering"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mycc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mycc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-engineering"
updated_at: "2026-10-16T18:59:18.151150516+00:00"
---

## Summary

cargo test -p astromech-engineering

## Command

```console
cargo test -p astromech-engineering
```

Exit status: 0

## Stdout

Bytes: 940
Truncated: no

```text

running 13 tests
test tests::farm_route_trades_down_from_anchors ... ok
test tests::trader_rates ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::fleet_from_history ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::catalogue_materials_resolve ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::trip_in_person_stops_and_remote_note ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 201
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ff71dd009829e65d)
   Doc-tests astromech_engineering
```

