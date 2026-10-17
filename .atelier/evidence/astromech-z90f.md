---
created_at: "2026-10-17T17:11:33.089944613+00:00"
id: "astromech-z90f"
evidence_type: "test"
commit: "c811a5be9b1a4cd717a2564b53bd05ec99d49962"
result: "pass"
captured_at: "2026-10-17T17:11:29.360099152+00:00"
command: "cargo test -p astromech-engineering -p astromech-journal"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-hnar"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-hnar"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-engineering -p astromech-journal"
updated_at: "2026-10-17T17:11:33.090267835+00:00"
---

## Summary

cargo test -p astromech-engineering -p astromech-journal

## Command

```console
cargo test -p astromech-engineering -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 3845
Truncated: no

```text

running 28 tests
test tests::farm_route_trades_down_from_anchors ... ok
test links::tests::coriolis_round_trips_the_loadout ... ok
test links::tests::edsy_round_trips_the_loadout ... ok
test links::tests::only_our_links_are_openable ... ok
test links::tests::urls_snapshot ... ok
test links::tests::large_loadout_urls_stay_openable_and_short ... ok
test tests::fetched_module_is_in_transit_until_its_transfer_time_passes ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::crafts_replace_the_modules_modifiers ... ok
test tests::engineer_grades_are_per_module ... ok
test tests::trader_rates ... ok
test tests::stored_modules_follow_store_retrieve_and_remote_sale ... ok
test tests::transferred_ship_arrives_after_its_transfer_time ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test tests::fleet_from_history ... ok
test tests::catalogue_materials_resolve ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::planned_modules_follow_the_plan_like_the_game ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::grades_remaining_and_shopping_list ... ok
test unlocks::tests::every_ship_and_suit_engineer_has_unlock_data ... ok
test unlocks::tests::tree_marks_progress_and_what_is_left ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::trip_skips_engineers_capped_below_the_grade_for_this_module ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok
test tests::trip_in_person_stops_and_remote_note ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 9 tests
test tests::docked_keeps_system_address ... ok
test tests::docked_reads_what_the_station_offers ... ok
test tests::cargo_and_route_files ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::known_and_unknown_events ... ok
test tests::positions_and_engineer_events ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::loadout_modules_and_fleet_events ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
test a_scans_asteroid_belts_are_not_rings ... ok
test cargo_inventory ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test docking_granted_and_denied ... ok
test jump_planning_events ... ok
test exobiology_events ... ok
test bounty_and_voucher ... ok
test fss_events ... ok
test colonisation_family ... ok
test exploration_sales ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test codex_settlement_and_beacon ... ok
test carrier_family ... ok
test combat_events ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test market_buy_and_sell ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test unknown_events_still_fall_through_to_other ... ok
test saa_events ... ok
test scan_body_and_barycentre ... ok
test statistics_flattens_numeric_counters ... ok
test mining ... ok
test supercruise_transitions ... ok
test ship_locker ... ok
test rank_progress_reputation ... ok
test system_powerplay_on_jump_and_location ... ok
test missions ... ok
test fuel_chat_and_surface_events ... ok
test powerplay_family ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 805
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-bugs-state/crates/astromech-journal)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bugs-state/crates/astromech-materials)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-bugs-state/crates/astromech-engineering)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.41s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-fdb4377f4b8a2adb)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-e10d090a739a949c)
     Running tests/events.rs (target/debug/deps/events-f2c83d08d19466ae)
   Doc-tests astromech_engineering
   Doc-tests astromech_journal
```

