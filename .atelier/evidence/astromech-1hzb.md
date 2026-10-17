---
created_at: "2026-10-17T17:36:21.803060042+00:00"
id: "astromech-1hzb"
evidence_type: "test"
commit: "8e9e55729ef3ed970763cefff04f6dc4cc1c43fc"
result: "pass"
captured_at: "2026-10-17T17:35:28.294670368+00:00"
command: "bash -c 'cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app && pnpm -s lint && pnpm -s check && pnpm -s build && pnpm -s test'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-5sbu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-5akj"
    role: "validates"
  - kind: "issue"
    id: "astromech-5sbu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fixes R1-R4, R8: CONTRIBUTING pre-PR checks at this commit (fmt --check, clippy -D warnings, cargo test --workspace, check-deps, pnpm lint/check/build/test). Includes route_follow unit tests (only live arrivals move the route; the toggle turns it off), the announce catch-up test (route_follow silent during catch-up, one line live), gear_down: the catch-up into a waypoint says and copies nothing and a live jump moves the route; a replayed older journal through waypoints leaves route and clipboard alone (the reviewer's R1 scenario); paste stops before Enter when the map closes. Mutation check: making replayed() call arrived() fails both route catch-up/replay tests."
updated_at: "2026-10-17T17:36:24.676154422+00:00"
---

## Summary

Review fixes R1-R4, R8: CONTRIBUTING pre-PR checks at this commit (fmt --check, clippy -D warnings, cargo test --workspace, check-deps, pnpm lint/check/build/test). Includes route_follow unit tests (only live arrivals move the route; the toggle turns it off), the announce catch-up test (route_follow silent during catch-up, one line live), gear_down: the catch-up into a waypoint says and copies nothing and a live jump moves the route; a replayed older journal through waypoints leaves route and clipboard alone (the reviewer's R1 scenario); paste stops before Enter when the map closes. Mutation check: making replayed() call arrived() fails both route catch-up/replay tests.

## Command

```console
bash -c 'cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app && pnpm -s lint && pnpm -s check && pnpm -s build && pnpm -s test'
```

Exit status: 0

## Stdout

Bytes: 81758
Truncated: yes

```text

running 6 tests
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test route_follow::tests::only_live_arrivals_move_the_route ... ok
test session_summary::tests::credits_round_into_millions_at_the_boundary ... ok
test route_follow::tests::the_toggle_turns_it_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
test a_system_visited_twice_before_says_so_and_when_last ... ok
test landings_lines ... ok
test a_body_landed_on_before_is_named_with_how_often ... ok
test a_new_system_says_nothing_about_visits ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test several_bodies_are_counted_and_none_says_nothing ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test how_long_ago_reads_in_the_nearest_unit ... ok
test the_visits_line_follows_the_state_line_within_the_word_budget ... ok
test the_visits_line_is_silent_in_the_catch_up_and_without_a_history ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test a_late_answer_awaited_in_one_system_is_not_said_in_the_next ... ok
test a_relog_into_a_system_already_scanned_says_nothing ... ok
test can_be_turned_off ... ok
test values_and_lines_read_well ... ok
test a_late_answer_is_said_on_the_tick_within_a_minute_and_once ... ok
test arrival_with_known_biology_says_it_before_any_scan ... ok
test a_carrier_jump_or_a_login_is_an_arrival_too ... ok
test each_system_starts_afresh ... ok
test the_summary_follows_the_fss_and_the_dss ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
test an_earth_like_world_gets_one_line_and_all_bodies_found_none ... ok
test terraformables_and_mapping_targets_are_called_out_and_the_rest_are_not ... ok
test it_can_be_turned_off ... ok
test nav_beacon_data_is_not_a_discovery_and_each_visit_starts_afresh ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test docking_callouts_are_silent_during_the_catch_up ... ok
test each_denial_reason_gets_its_line ... ok
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok
test a_coriolis_pad_number_gives_its_position ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test enough_fuel_gives_no_warning ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test hge_callouts_are_silent_during_the_catch_up ... ok
test other_signals_stay_quiet ... ok
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test each_spawning_faction_gives_its_own_pool ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_hge_system_on_arrival ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

## Stderr

Bytes: 9224
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
warning: output filename collision at /home/supadava/projects/astromech-wt-route/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-route/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-cbdeb8ce98fd0ca4)
     Running tests/arrival.rs (target/debug/deps/arrival-a8e69e0aa9414fe6)
     Running tests/biology.rs (target/debug/deps/biology-692807976b3f27ae)
     Running tests/discovery.rs (target/debug/deps/discovery-958a4074bf7c3c0d)
     Running tests/docking.rs (target/debug/deps/docking-2b284014f6d8e11c)
     Running tests/fuel.rs (target/debug/deps/fuel-0e1d72f6a5db29e4)
     Running tests/hge.rs (target/debug/deps/hge-512491ec1a3b25d2)
     Running tests/materials_cap.rs (target/debug/deps/materials_cap-c7b58ed4d756fb7e)
     Running tests/missions.rs (target/debug/deps/missions-7b6182039d4a6df2)
     Running tests/powerplay_arrival.rs (target/debug/deps/powerplay_arrival-68baceeb05e0dfb5)
     Running tests/project_market.rs (target/debug/deps/project_market-f5cefa52980282ba)
     Running tests/prospector.rs (target/debug/deps/prospector-18905f74835be535)
     Running tests/replay.rs (target/debug/deps/replay-23e1982a7ae087c2)
     Running tests/route.rs (target/debug/deps/route-6e73e058b2763c81)
     Running tests/sampling.rs (target/debug/deps/sampling-39c52452a0067e94)
     Running tests/scan.rs (target/debug/deps/scan-70cc3edf5a4a69f2)
     Running tests/session_summary.rs (target/debug/deps/session_summary-1acfcb15dd6a9b28)
     Running tests/signals.rs (target/debug/deps/signals-a16a7ed649fc5e05)
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-2d2d79e035a050ea)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-0c62a4db5821b824)
     Running unittests src/main.rs (target/debug/deps/astromech_core-46ccd4c8369352b8)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-98e2964ba99c6392)
     Running tests/copy.rs (target/debug/deps/copy-7ddf46b04899ffd2)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-4d98ca24ede1db4a)
     Running tests/gear_down.rs (target/debug/deps/gear_down-eea25555e1795f14)
     Running tests/mining_status.rs (target/debug/deps/mining_status-91a8ca16fef8185a)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-0fbc96bada454301)
     Running tests/session_summary.rs (target/debug/deps/session_summary-e5f2c2e190ee8d8d)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-efa5ba882cb8e6e1)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-4eda110adb629a73)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-ffaf72b6ad461eed)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-6ed659280f0eb760)
     Running tests/recorded.rs (t
```

