---
created_at: "2026-10-17T17:29:12.979764294+00:00"
id: "astromech-qt9o"
evidence_type: "test"
commit: "e4bf4dcdb54bb9f39749a7065e9870c904270efe"
result: "pass"
captured_at: "2026-10-17T17:29:11.023223308+00:00"
command: "cargo test -p astromech-journal -p astromech-engineering --tests"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ztrb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ztrb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-journal -p astromech-engineering --tests"
updated_at: "2026-10-17T17:29:12.980038956+00:00"
---

## Summary

cargo test -p astromech-journal -p astromech-engineering --tests

## Command

```console
cargo test -p astromech-journal -p astromech-engineering --tests
```

Exit status: 0

## Stdout

Bytes: 4123
Truncated: yes

```text

running 25 tests
test tests::farm_route_trades_down_from_anchors ... ok
test links::tests::coriolis_round_trips_the_loadout ... ok
test links::tests::only_our_links_are_openable ... ok
test links::tests::edsy_round_trips_the_loadout ... ok
test links::tests::urls_snapshot ... ok
test links::tests::large_loadout_urls_stay_openable_and_short ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::crafts_replace_the_modules_modifiers ... ok
test tests::engineer_grades_are_per_module ... ok
test tests::trader_rates ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::catalogue_materials_resolve ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::planned_modules_follow_the_plan_like_the_game ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::fleet_from_history ... ok
test unlocks::tests::tree_marks_progress_and_what_is_left ... ok
test unlocks::tests::every_ship_and_suit_engineer_has_unlock_data ... ok
test tests::trip_skips_engineers_capped_below_the_grade_for_this_module ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::trip_in_person_stops_and_remote_note ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 1 test
test appends_to_an_open_journal_reach_the_pages_on_the_next_call ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 9 tests
test tests::docked_keeps_system_address ... ok
test tests::cargo_and_route_files ... ok
test tests::docked_reads_what_the_station_offers ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::known_and_unknown_events ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::positions_and_engineer_events ... ok
test tests::loadout_modules_and_fleet_events ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
test a_scans_asteroid_belts_are_not_rings ... ok
test cargo_inventory ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test docking_granted_and_denied ... ok
test codex_settlement_and_beacon ... ok
test jump_planning_events ... ok
test bounty_and_voucher ... ok
test exobiology_events ... ok
test exploration_sales ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test fss_events ... ok
test market_buy_and_sell ... ok
test colonisation_family ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test saa_events ... ok
test scan_body_and_barycentre ... ok
test carrier_family ... ok
test combat_events ... ok
test mining ... ok
test fuel_chat_and_surface_events ... ok
test powerplay_family ... ok
test missions ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test unknown_events_still_fall_through_to_other ... ok
test supercruise_transitions ... ok
test rank_progress_reputation ... ok
test statistics_flattens_numeric_counters ... ok
test ship_locker ... ok
test system_powerplay_on_jump_and_location ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 7 tests
test a_first_poll_that_cannot_open_the_journal_does_not_pin_the_build ... ok
test a_half_written_line_counts_once_it_is_finished ... ok
test a_panicking_build_does_not_wedge_the_cache ... ok
test a_new_journal_file_counts ... ok
test a_new_journal_that_fails_to_open_is_counted_when_it_can_be_read ... ok
test a_cache_rebuilds_for_modelled_events_only ... ok
test typed_appends_to_an_open_journal_change_the_generation ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filter
```

## Stderr

Bytes: 726
Truncated: no

```text
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-journal)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-engineering)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.78s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-60607fa2eb988779)
     Running tests/live_journal.rs (target/debug/deps/live_journal-2f4b4012ac6143bf)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-e10d090a739a949c)
     Running tests/events.rs (target/debug/deps/events-f2c83d08d19466ae)
     Running tests/live_watch.rs (target/debug/deps/live_watch-9983733a96279bbd)
```

