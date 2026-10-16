---
created_at: "2026-10-16T18:36:06.102370453+00:00"
id: "astromech-db4n"
evidence_type: "test"
commit: "ec786143c4ef8937bd6e6a33cd72c802883e6fd9"
result: "pass"
captured_at: "2026-10-16T18:36:02.265181301+00:00"
command: "cargo test -p astromech-journal"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-qgjp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qgjp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "28 per-event parser tests on synthetic fixtures plus raw JSON byte-for-byte tests"
updated_at: "2026-10-16T18:36:06.102861989+00:00"
---

## Summary

28 per-event parser tests on synthetic fixtures plus raw JSON byte-for-byte tests

## Command

```console
cargo test -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 1756
Truncated: no

```text

running 7 tests
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::loadout_modules_and_fleet_events ... ok
test tests::cargo_and_route_files ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::known_and_unknown_events ... ok
test tests::positions_and_engineer_events ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 28 tests
test bounty_and_voucher ... ok
test cargo_inventory ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test exploration_sales ... ok
test exobiology_events ... ok
test carrier_family ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test colonisation_family ... ok
test market_buy_and_sell ... ok
test fss_events ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test mining ... ok
test saa_events ... ok
test codex_settlement_and_beacon ... ok
test jump_planning_events ... ok
test unknown_events_still_fall_through_to_other ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test docking_granted_and_denied ... ok
test statistics_flattens_numeric_counters ... ok
test scan_body_and_barycentre ... ok
test supercruise_transitions ... ok
test missions ... ok
test ship_locker ... ok
test combat_events ... ok
test powerplay_family ... ok
test fuel_chat_and_surface_events ... ok
test rank_progress_reputation ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 391
Truncated: no

```text
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tpdb/crates/astromech-journal)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.32s
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-7895683a32eb8dbe)
     Running tests/events.rs (target/debug/deps/events-030d4df97a9204fd)
   Doc-tests astromech_journal
```

