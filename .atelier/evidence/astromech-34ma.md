---
created_at: "2026-10-17T02:43:28.470620482+00:00"
id: "astromech-34ma"
evidence_type: "test"
commit: "d8845159e808181b9c19d8a775af7a4cbaca0217"
result: "pass"
captured_at: "2026-10-17T02:43:03.947952944+00:00"
command: "cargo test -p astromech-announce -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-2myu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-2myu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Replay tests: twice-visited system gets the line, new system and catch-up are silent (fail with the line disabled); history query and copy tests; fmt, clippy, workspace tests, check-deps, tauri metadata green"
updated_at: "2026-10-17T02:43:28.471229147+00:00"
---

## Summary

Replay tests: twice-visited system gets the line, new system and catch-up are silent (fail with the line disabled); history query and copy tests; fmt, clippy, workspace tests, check-deps, tauri metadata green

## Command

```console
cargo test -p astromech-announce -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 13684
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 13 tests
test a_new_system_says_nothing_about_visits ... ok
test a_system_visited_twice_before_says_so_and_when_last ... ok
test how_long_ago_reads_in_the_nearest_unit ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_visits_line_follows_the_state_line_within_the_word_budget ... ok
test the_visits_line_is_silent_in_the_catch_up_and_without_a_history ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test docking_callouts_are_silent_during_the_catch_up ... ok
test a_coriolis_pad_number_gives_its_position ... ok
test each_denial_reason_gets_its_line ... ok
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test enough_fuel_gives_no_warning ... ok
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test hge_callouts_are_silent_during_the_catch_up ... ok
test other_signals_stay_quiet ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival ... ok
test each_spawning_faction_gives_its_own_pool ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_rock_above_threshold_gets_one_short_line ... ok
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_motherlode_always_gets_a_line ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test catch_up_leaves_docking_quiet_in_place ... ok
test bad_settings_are_reported_not_fatal ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test a_disabled_rule_never_fires ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test a_plain_route_gets_only_its_summary ... ok
test the_fuel_warning_is_not_said_again_on_the_next_arrival ... ok
test the_route_briefing_is_silent_during_the_catch_up ... ok
test a_white_dwarf_and_a_fuel_gap_give_one_hazard_line_each ... ok
test a_routes_fuel_gap_does_not_silence_an_arrival_warning ... ok
test an_arrival_warning_does_not_silence_a_new_routes_fuel_gap ... ok
test the_same_route_w
```

## Stderr

Bytes: 1849
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Blocking waiting for file lock on package cache
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-2myu/crates/astromech-announce)
   Compiling astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-2myu/crates/astromech-history)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-2myu/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.84s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-18783c3125c489e8)
     Running tests/arrival.rs (target/debug/deps/arrival-b836d84e6779b5a3)
     Running tests/docking.rs (target/debug/deps/docking-a0d54b05eb356677)
     Running tests/fuel.rs (target/debug/deps/fuel-70c55c2529192d1c)
     Running tests/hge.rs (target/debug/deps/hge-824d8735b1c8a321)
     Running tests/prospector.rs (target/debug/deps/prospector-c3f3ef8d5bde88cd)
     Running tests/replay.rs (target/debug/deps/replay-527bb8fd1f69a5f3)
     Running tests/route.rs (target/debug/deps/route-f9fe31f66563ab7b)
     Running tests/scan.rs (target/debug/deps/scan-fd059ed07f5afdeb)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-57f329e875b1cf93)
     Running unittests src/main.rs (target/debug/deps/astromech_core-3d993b773572a817)
     Running tests/copy.rs (target/debug/deps/copy-de1e57b414a1ad02)
     Running tests/gear_down.rs (target/debug/deps/gear_down-b7928756ed9d512e)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-e7e9091d2f7d0a4b)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-532abdc895661d64)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-da856e0a3530f7d6)
   Doc-tests astromech_announce
   Doc-tests astromech_core
```

