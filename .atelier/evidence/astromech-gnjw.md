---
created_at: "2026-10-17T02:53:22.416056444+00:00"
id: "astromech-gnjw"
evidence_type: "test"
commit: "34b8f5e91af6c8327a9dd2fc32e2dddd25493858"
result: "pass"
captured_at: "2026-10-17T02:53:00.498298705+00:00"
command: "cargo test -p astromech-announce -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ze13"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ze13"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-announce -p astromech-core"
updated_at: "2026-10-17T02:53:22.416330576+00:00"
---

## Summary

cargo test -p astromech-announce -p astromech-core

## Command

```console
cargo test -p astromech-announce -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 13427
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test tests::settings_read_from_toml ... ok
test prospector::tests::thresholds_default_override_and_off ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok
test docking_callouts_are_silent_during_the_catch_up ... ok
test a_coriolis_pad_number_gives_its_position ... ok
test each_denial_reason_gets_its_line ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 10 tests
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test enough_fuel_gives_no_warning ... ok
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test hge_callouts_are_silent_during_the_catch_up ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test each_spawning_faction_gives_its_own_pool ... ok
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test other_signals_stay_quiet ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test catch_up_is_silent_even_when_full ... ok
test docking_at_a_trader_names_what_to_trade_down ... ok
test filling_a_material_says_so_once ... ok
test the_trader_line_speaks_after_an_approach ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test a_motherlode_always_gets_a_line ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_rock_below_threshold_gets_nothing ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test a_disabled_rule_never_fires ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok
test bad_settings_are_reported_not_fatal ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test a_plain_route_gets_only_its_summary ... ok
test without_a_jump_to_go_by_there_is_no_fuel_line ... ok
test the_route_briefing_is_silent_during_the_catch_up ... ok
test the_same_route_written_again_is_not_said_again_and_a_cleared_route_is_quiet ... ok
test too_little_fuel_for_the_first_scoopable_star_is_a_fuel_gap_too ... ok
test a_white_dwarf_and_a_fuel_gap_give_one_hazard_line_each ... ok
test an_
```

## Stderr

Bytes: 1762
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-ze13/crates/astromech-announce)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-ze13/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.57s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-57803e48f9217f6c)
     Running tests/arrival.rs (target/debug/deps/arrival-0f9ae8ccfdeb11df)
     Running tests/docking.rs (target/debug/deps/docking-f6328dd26da4f083)
     Running tests/fuel.rs (target/debug/deps/fuel-77d59fb215bff780)
     Running tests/hge.rs (target/debug/deps/hge-26a8b386a15db250)
     Running tests/materials_cap.rs (target/debug/deps/materials_cap-d05ea71a0844f20e)
     Running tests/prospector.rs (target/debug/deps/prospector-b2ec4c9b72ba5b4c)
     Running tests/replay.rs (target/debug/deps/replay-268d4dfdd373d3ff)
     Running tests/route.rs (target/debug/deps/route-83c20ecf7e30ebfb)
     Running tests/scan.rs (target/debug/deps/scan-cd0fd7603906922a)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-38eef3c8cf4fd98c)
     Running unittests src/main.rs (target/debug/deps/astromech_core-defc8f5acd32b7b5)
     Running tests/copy.rs (target/debug/deps/copy-31003c62a4b6019e)
     Running tests/gear_down.rs (target/debug/deps/gear_down-016202d7a7f403d8)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-a2079c820e457e8d)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-5ade06d95369fdec)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-e561dc84611ee890)
   Doc-tests astromech_announce
   Doc-tests astromech_core
```

