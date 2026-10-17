---
created_at: "2026-10-17T02:00:59.888757407+00:00"
id: "astromech-21jd"
evidence_type: "test"
commit: "2eaba610a1f15e9e5e5b2d611576cb1f9f03d0cc"
result: "pass"
captured_at: "2026-10-17T02:00:56.104660866+00:00"
command: "bash -c 'cargo test -p astromech-announce --test hge --test arrival --test replay && cargo test -p astromech-journal && cargo test -p astromech-core --test copy'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-y916"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-y916"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "HGE replay tests (crates/astromech-announce/tests/hge.rs): an HGE spawned by a Federation faction in Famine names Federation composites (not the Boom faction's Proto pool); each spawning faction gives its own pool, an unknown spawner the most influential faction's, a wanted material is named instead of the pool, a faction with no pool gives 'High Grade Emissions.'; non-human, encoded, station and conflict-zone signals stay quiet; a burst from one honk is said once; on arrival a 2.5 billion Empire system with Imperial Shielding wanted is 'Good for Imperial Shielding.', under a billion or not wanted is quiet; catch-up silent. 4 of 6 failed before the rule existed. Journal test parses the game's HGE shape with SpawningFaction. Copy guide covers every line."
updated_at: "2026-10-17T02:00:59.889045113+00:00"
---

## Summary

HGE replay tests (crates/astromech-announce/tests/hge.rs): an HGE spawned by a Federation faction in Famine names Federation composites (not the Boom faction's Proto pool); each spawning faction gives its own pool, an unknown spawner the most influential faction's, a wanted material is named instead of the pool, a faction with no pool gives 'High Grade Emissions.'; non-human, encoded, station and conflict-zone signals stay quiet; a burst from one honk is said once; on arrival a 2.5 billion Empire system with Imperial Shielding wanted is 'Good for Imperial Shielding.', under a billion or not wanted is quiet; catch-up silent. 4 of 6 failed before the rule existed. Journal test parses the game's HGE shape with SpawningFaction. Copy guide covers every line.

## Command

```console
bash -c 'cargo test -p astromech-announce --test hge --test arrival --test replay && cargo test -p astromech-journal && cargo test -p astromech-core --test copy'
```

Exit status: 0

## Stdout

Bytes: 3623
Truncated: no

```text

running 7 tests
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test hge_callouts_are_silent_during_the_catch_up ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test other_signals_stay_quiet ... ok
test each_spawning_faction_gives_its_own_pool ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test a_disabled_rule_never_fires ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test quiet_during_docking ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_combat ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test tests::known_and_unknown_events ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::docked_keeps_system_address ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::loadout_modules_and_fleet_events ... ok
test tests::positions_and_engineer_events ... ok
test tests::cargo_and_route_files ... ok
test tests::docked_reads_what_the_station_offers ... ok
test tests::journal_names_order_across_schemes ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 29 tests
test cargo_inventory ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test docking_granted_and_denied ... ok
test bounty_and_voucher ... ok
test exobiology_events ... ok
test jump_planning_events ... ok
test exploration_sales ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test fss_events ... ok
test market_buy_and_sell ... ok
test codex_settlement_and_beacon ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test colonisation_family ... ok
test supercruise_transitions ... ok
test scan_body_and_barycentre ... ok
test carrier_family ... ok
test unknown_events_still_fall_through_to_other ... ok
test ship_locker ... ok
test mining ... ok
test statistics_flattens_numeric_counters ... ok
test combat_events ... ok
test powerplay_family ... ok
test fuel_chat_and_surface_events ... ok
test missions ... ok
test saa_events ... ok
test system_powerplay_on_jump_and_location ... ok
test rank_progress_reputation ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 2331
Truncated: no

```text
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-announce)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running tests/arrival.rs (target/debug/deps/arrival-c75cf16e643b54dc)
     Running tests/hge.rs (target/debug/deps/hge-f95d9af9b7cc1f3b)
     Running tests/replay.rs (target/debug/deps/replay-47133f362d1a518e)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-e10d090a739a949c)
     Running tests/events.rs (target/debug/deps/events-f2c83d08d19466ae)
   Doc-tests astromech_journal
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-journal)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-materials)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-powerplay)
   Compiling astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-history)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-gamestate)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-galaxy)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-engineering)
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-announce)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-market)
   Compiling astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-station)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.99s
     Running tests/copy.rs (target/debug/deps/copy-31003c62a4b6019e)
```

