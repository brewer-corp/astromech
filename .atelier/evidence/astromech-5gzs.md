---
created_at: "2026-10-17T22:21:36.527055619+00:00"
id: "astromech-5gzs"
evidence_type: "test"
commit: "7057786a651ce275bd6300998060f66e773852c2"
result: "pass"
captured_at: "2026-10-17T22:21:17.620117987+00:00"
command: "cargo test -p astromech-announce -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-bh0c"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-bh0c"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-announce -p astromech-core"
updated_at: "2026-10-17T22:21:36.527419557+00:00"
---

## Summary

cargo test -p astromech-announce -p astromech-core

## Command

```console
cargo test -p astromech-announce -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 27626
Truncated: yes

```text

running 8 tests
test catalog_tests::every_built_in_rule_has_a_title_and_a_default ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test route_follow::tests::only_live_arrivals_move_the_route ... ok
test known_bodies::tests::lines ... ok
test tests::settings_read_from_toml ... ok
test route_follow::tests::the_toggle_turns_following_off_but_not_where_you_are ... ok
test session_summary::tests::credits_round_into_millions_at_the_boundary ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
test a_new_system_says_nothing_about_visits ... ok
test a_body_landed_on_before_is_named_with_how_often ... ok
test a_system_visited_twice_before_says_so_and_when_last ... ok
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test landings_lines ... ok
test how_long_ago_reads_in_the_nearest_unit ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test several_bodies_are_counted_and_none_says_nothing ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_visits_line_follows_the_state_line_within_the_word_budget ... ok
test the_visits_line_is_silent_in_the_catch_up_and_without_a_history ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test a_relog_into_a_system_already_scanned_says_nothing ... ok
test a_late_answer_awaited_in_one_system_is_not_said_in_the_next ... ok
test can_be_turned_off ... ok
test a_late_answer_is_said_on_the_tick_within_a_minute_and_once ... ok
test a_carrier_jump_or_a_login_is_an_arrival_too ... ok
test arrival_with_known_biology_says_it_before_any_scan ... ok
test each_system_starts_afresh ... ok
test values_and_lines_read_well ... ok
test the_summary_follows_the_fss_and_the_dss ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
test it_can_be_turned_off ... ok
test an_earth_like_world_gets_one_line_and_all_bodies_found_none ... ok
test nav_beacon_data_is_not_a_discovery_and_each_visit_starts_afresh ... ok
test terraformables_and_mapping_targets_are_called_out_and_the_rest_are_not ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test docking_callouts_are_silent_during_the_catch_up ... ok
test each_denial_reason_gets_its_line ... ok
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok
test a_coriolis_pad_number_gives_its_position ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test enough_fuel_gives_no_warning ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival ... ok
test other_signals_stay_quiet ... ok
test hge_callouts_are_silen
```

## Stderr

Bytes: 2755
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-77a4cb6ae55b27ed)
     Running tests/arrival.rs (target/debug/deps/arrival-ffd2e7d3763bb4a3)
     Running tests/biology.rs (target/debug/deps/biology-3e9dd172a1c0c767)
     Running tests/discovery.rs (target/debug/deps/discovery-6c73a576acd56bde)
     Running tests/docking.rs (target/debug/deps/docking-5cc98d0cb7a70d35)
     Running tests/fuel.rs (target/debug/deps/fuel-d9282160778b62d8)
     Running tests/hge.rs (target/debug/deps/hge-e75f107290c91870)
     Running tests/materials_cap.rs (target/debug/deps/materials_cap-79900d76e76ca530)
     Running tests/missions.rs (target/debug/deps/missions-de5e68c79aa38f65)
     Running tests/powerplay_arrival.rs (target/debug/deps/powerplay_arrival-357185c8bb676302)
     Running tests/project_market.rs (target/debug/deps/project_market-1f933952a8568e88)
     Running tests/prospector.rs (target/debug/deps/prospector-f7eae536dc6d06f8)
     Running tests/replay.rs (target/debug/deps/replay-0a310fe0539a258d)
     Running tests/route.rs (target/debug/deps/route-b35224ee7bb298c9)
     Running tests/sampling.rs (target/debug/deps/sampling-7b5b0a27252b946a)
     Running tests/scan.rs (target/debug/deps/scan-077afb3eed9d281d)
     Running tests/session_summary.rs (target/debug/deps/session_summary-26fb7e15e39fbe24)
     Running tests/signals.rs (target/debug/deps/signals-71f94a61e3da19f4)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-4b0fe231a2c4fa9d)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8d41cff28f33e414)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-d1e597292f57cf6b)
     Running tests/copy.rs (target/debug/deps/copy-ec2bf691432caef5)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-4f346e0670592eab)
     Running tests/gear_down.rs (target/debug/deps/gear_down-a7a1a38880465a4b)
     Running tests/known_bodies.rs (target/debug/deps/known_bodies-b1853a3e0ef4861d)
     Running tests/mining_status.rs (target/debug/deps/mining_status-1765caa241d80906)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-a31e6452f9d05ace)
     Running tests/session_summary.rs (target/debug/deps/session_summary-86a91f164548ef94)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-6e5d1c133b77b336)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-e34f6a6eb996ba8a)
     Running tests/voice_eval_phrases.rs (target/debug/deps/voice_eval_phrases-e9179e47821d18a4)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-9c6abb7368909561)
   Doc-tests astromech_announce
   Doc-tests astromech_core
```

