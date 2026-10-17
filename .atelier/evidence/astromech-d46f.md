---
created_at: "2026-10-17T02:42:30.420676874+00:00"
id: "astromech-d46f"
evidence_type: "test"
commit: "d2d372ada9482b3c0824fb7b942e9de68891b48f"
result: "pass"
captured_at: "2026-10-17T02:42:06.657164014+00:00"
command: "cargo test --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-gtza"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-gtza"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "powerplay_arrival replay tests (on, off, catch-up), copy guide, workspace tests pass"
updated_at: "2026-10-17T02:42:30.421665868+00:00"
---

## Summary

powerplay_arrival replay tests (on, off, catch-up), copy guide, workspace tests pass

## Command

```console
cargo test --workspace
```

Exit status: 0

## Stdout

Bytes: 48935
Truncated: yes

```text

running 3 tests
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test an_undiscovered_system_is_said_after_the_jump ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test docking_callouts_are_silent_during_the_catch_up ... ok
test each_denial_reason_gets_its_line ... ok
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok
test a_coriolis_pad_number_gives_its_position ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test enough_fuel_gives_no_warning ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test hge_callouts_are_silent_during_the_catch_up ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival ... ok
test each_spawning_faction_gives_its_own_pool ... ok
test other_signals_stay_quiet ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
test a_long_list_is_said_a_clause_at_a_time ... ok
test a_login_location_is_not_an_arrival ... ok
test a_system_without_powerplay_activity_is_silent ... ok
test is_silent_during_the_catch_up ... ok
test is_off_by_default ... ok
test speaks_the_systems_powerplay_state_when_on ... ok
test the_arrival_rule_does_not_repeat_the_target_when_on ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_motherlode_always_gets_a_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test threshold_changes_from_settings_apply_at_once ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test a_disabled_rule_never_fires ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test a_plain_route_gets_only_its_summary ... ok
test a_white_dwarf_and_a_fuel_gap_give_one_hazard_line_each ... ok
test a_routes_fuel_gap_does_not_silence_an_arrival_warning ... ok
test an_arrival_warning_does_not_silence_a_new_routes_fuel_gap ... ok
test th
```

## Stderr

Bytes: 7001
Truncated: yes

```text
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-gtza/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-gtza/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-gtza/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-gtza/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-gtza/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-gtza/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-81f864eb12072685)
     Running tests/arrival.rs (target/debug/deps/arrival-01cf1d1942c27ac5)
     Running tests/docking.rs (target/debug/deps/docking-d4f9762d04da9108)
     Running tests/fuel.rs (target/debug/deps/fuel-78ac381a914e8bd4)
     Running tests/hge.rs (target/debug/deps/hge-bd4998a815372d51)
     Running tests/powerplay_arrival.rs (target/debug/deps/powerplay_arrival-c85fcc841c386596)
     Running tests/prospector.rs (target/debug/deps/prospector-f45a662a3b2fe894)
     Running tests/replay.rs (target/debug/deps/replay-c0033618d4fb6072)
     Running tests/route.rs (target/debug/deps/route-0ea552cd61689573)
     Running tests/scan.rs (target/debug/deps/scan-73eab6201992c7d9)
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-9ee2fa666954b1d4)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3e84665cc9f02cab)
     Running unittests src/main.rs (target/debug/deps/astromech_core-f3bbe97e67a56ecd)
     Running tests/copy.rs (target/debug/deps/copy-8636fe10a954dfea)
     Running tests/gear_down.rs (target/debug/deps/gear_down-3f6ef0d7e41767b6)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-8e1d84ed972025e1)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-74b6d18ef8b3b1aa)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-384c5bf52fd77c54)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-6ed659280f0eb760)
     Running tests/recorded.rs (target/debug/deps/recorded-077bd2d8d758312c)
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-943ca53e72b74208)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ccdb1e86be63f36e)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-5811f7cd0f3a0534)
     Running tests/powerplay.rs (target/debug/deps/powerplay-279087c75a955c55)
     Running tests/sites.rs (target/debug/deps/sites-61e89b44860c1b5a)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-5296df5f07f4fb66)
     Running tests/companions.rs (target/debug/deps/companions-47219eed008c5014)
     Running tests/mining.rs (target/debug/deps/mining-24f0a0d64f8ca403)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-24c9bd3f58aacc1d)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-71818ae170a2b90a)
     Running tests/listen.rs (target/debug/deps/listen-aefcf94f21ae37fb)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-49143e7857eb60de)
     Running unittests src/lib.rs (target/debug/
```

