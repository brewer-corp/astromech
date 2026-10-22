---
created_at: "2026-10-22T21:31:02.168962442+00:00"
id: "astromech-4esx"
evidence_type: "test"
commit: "dfcb12fd136d61438891084e3e8672dc52cabd6c"
result: "pass"
captured_at: "2026-10-22T21:30:57.693278971+00:00"
command: "cargo test -p astromech-engineering -p astromech-ship --lib"
exit_status: "0"
agent_identity: "claude-implementer-fdh3"
target:
  kind: "issue"
  id: "astromech-ikwr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ikwr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Same-grade fully rolled target with same experimental keeps journal Modifiers (not fully rolled: dropped); DSS_PatchRadius maps to proberadius, no unapplied labels; ship and engineering suites pass"
updated_at: "2026-10-22T21:31:02.169455772+00:00"
---

## Summary

Same-grade fully rolled target with same experimental keeps journal Modifiers (not fully rolled: dropped); DSS_PatchRadius maps to proberadius, no unapplied labels; ship and engineering suites pass

## Command

```console
cargo test -p astromech-engineering -p astromech-ship --lib
```

Exit status: 0

## Stdout

Bytes: 3309
Truncated: no

```text

running 35 tests
test links::tests::coriolis_round_trips_the_loadout ... ok
test links::tests::only_our_links_are_openable ... ok
test links::tests::edsy_round_trips_the_loadout ... ok
test links::tests::urls_snapshot ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test links::tests::large_loadout_urls_stay_openable_and_short ... ok
test tests::fetched_module_is_in_transit_until_its_transfer_time_passes ... ok
test tests::a_shipyard_listing_a_ship_at_a_station_ends_its_transfer ... ok
test tests::a_shipyard_reopened_mid_transfer_keeps_the_arrival ... ok
test tests::a_refreshed_module_list_keeps_a_transfer_under_way ... ok
test tests::crafts_replace_the_modules_modifiers ... ok
test tests::engineer_grades_are_per_module ... ok
test tests::catalogue_materials_resolve ... ok
test tests::mass_store_and_buy_keep_the_stored_list_current ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::a_journal_folder_replays_the_stored_events ... ok
test tests::stored_modules_follow_store_retrieve_and_remote_sale ... ok
test tests::trader_rates ... ok
test tests::a_swapped_out_module_keeps_its_engineering ... ok
test tests::remote_sale_and_fetch_follow_the_storage_slot ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::fleet_from_history ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test tests::transferred_ship_arrives_after_its_transfer_time ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::planned_modules_follow_the_plan_like_the_game ... ok
test unlocks::tests::every_ship_and_suit_engineer_has_unlock_data ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok
test tests::trip_skips_engineers_capped_below_the_grade_for_this_module ... ok
test unlocks::tests::tree_marks_progress_and_what_is_left ... ok
test tests::trip_in_person_stops_and_remote_note ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s


running 13 tests
test tests::capacitor_recharge_follows_the_pip_curve ... ok
test tests::unknown_ship_is_an_error ... ok
test tests::unpowered_drive_has_no_range ... ok
test module::tests::rail_gun_sustained_counts_its_charge ... ok
test module::tests::rail_gun_sustained_draw_uses_the_same_cycle_as_sustained_damage ... ok
test module::tests::an_engineered_reload_below_the_fire_interval_never_shows_sustained_above_burst ... ok
test module::tests::modifiers_set_the_stats_and_unknown_labels_are_reported ... ok
test module::tests::a_weapon_without_charge_with_a_tiny_reload_keeps_sustained_draw_and_damage_within_burst ... ok
test module::tests::unknown_grade_is_reported ... ok
test tests::unknown_modules_and_blueprints_are_reported_not_counted ... ok
test module::tests::grade_5_at_roll_0_is_the_grade_4_maximum ... ok
test tests::unknown_hull_is_incomplete_not_an_error ... ok
test module::tests::sustained_never_exceeds_burst_for_any_catalogue_module ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 431
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-ship v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-fdh3/crates/astromech-ship)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-c70eb7925250528f)
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-305e8ab15cf056ba)
```

