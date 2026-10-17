---
created_at: "2026-10-17T22:26:10.790459618+00:00"
id: "astromech-vtdv"
evidence_type: "test"
commit: "4892581f864dbe692cbbb0fcfe38de9ac8558db8"
result: "pass"
captured_at: "2026-10-17T22:26:06.882770594+00:00"
command: "cargo test -p astromech-engineering -p astromech-journal"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-4irh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-4irh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-engineering -p astromech-journal"
updated_at: "2026-10-17T22:26:10.790805223+00:00"
---

## Summary

cargo test -p astromech-engineering -p astromech-journal

## Command

```console
cargo test -p astromech-engineering -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 5606
Truncated: yes

```text

running 35 tests
test links::tests::coriolis_round_trips_the_loadout ... ok
test links::tests::only_our_links_are_openable ... ok
test links::tests::edsy_round_trips_the_loadout ... ok
test links::tests::urls_snapshot ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test tests::a_shipyard_listing_a_ship_at_a_station_ends_its_transfer ... ok
test tests::a_refreshed_module_list_keeps_a_transfer_under_way ... ok
test tests::fetched_module_is_in_transit_until_its_transfer_time_passes ... ok
test tests::engineer_grades_are_per_module ... ok
test tests::crafts_replace_the_modules_modifiers ... ok
test tests::mass_store_and_buy_keep_the_stored_list_current ... ok
test tests::catalogue_materials_resolve ... ok
test tests::a_journal_folder_replays_the_stored_events ... ok
test tests::a_swapped_out_module_keeps_its_engineering ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::a_shipyard_reopened_mid_transfer_keeps_the_arrival ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::stored_modules_follow_store_retrieve_and_remote_sale ... ok
test tests::remote_sale_and_fetch_follow_the_storage_slot ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::trader_rates ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test links::tests::large_loadout_urls_stay_openable_and_short ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::fleet_from_history ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::transferred_ship_arrives_after_its_transfer_time ... ok
test unlocks::tests::every_ship_and_suit_engineer_has_unlock_data ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::grades_remaining_and_shopping_list ... ok
test unlocks::tests::tree_marks_progress_and_what_is_left ... ok
test tests::trip_skips_engineers_capped_below_the_grade_for_this_module ... ok
test tests::planned_modules_follow_the_plan_like_the_game ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok
test tests::trip_in_person_stops_and_remote_note ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 1 test
test appends_to_an_open_journal_reach_the_pages_on_the_next_call ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 1 test
test a_ship_and_a_module_arrive_as_the_clock_passes_their_transfer_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 14 tests
test tail::tests::a_file_that_shrinks_is_read_again_from_the_start ... ok
test commander::tests::keys ... ok
test tests::docked_keeps_system_address ... ok
test tail::tests::tail_keeps_partial_lines ... ok
test tail::tests::read_to_stops_at_the_limit_and_read_new_takes_the_rest ... ok
test tests::cargo_and_route_files ... ok
test tests::docked_reads_what_the_station_offers ... ok
test tests::known_and_unknown_events ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::positions_and_engineer_events ... ok
test tests::the_latest_journal_is_by_the_stamp_in_its_name_and_never_a_conflict_copy ... ok
test tests::loadout_modules_and_fleet_events ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
test a_scans_asteroid_belts_are_not_rings ... ok
test cargo_inventory ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test docking_granted_and_denied ... ok
test fss_events ... ok
test bounty_and_voucher ... ok
test codex_settlement_and_beacon ... ok
test colonisation_family ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test combat_events ... ok
test carrier_family ... ok
test exobiology_events ... ok
test exploration_sales ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test market_bu
```

## Stderr

Bytes: 871
Truncated: no

```text
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-journal)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-engineering)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.43s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-6848589fb46c6461)
     Running tests/live_journal.rs (target/debug/deps/live_journal-00e9efb7c78c8078)
     Running tests/transfers.rs (target/debug/deps/transfers-17df3ffdb90cffc6)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-38a25dae1a927b41)
     Running tests/events.rs (target/debug/deps/events-8f31077cb0e5ef0a)
     Running tests/live_watch.rs (target/debug/deps/live_watch-11ed4039893bc232)
   Doc-tests astromech_engineering
   Doc-tests astromech_journal
```

