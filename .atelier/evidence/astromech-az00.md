---
created_at: "2026-10-23T00:06:39.437187919+00:00"
id: "astromech-az00"
evidence_type: "test"
commit: "e6ba59190363e27f8f94fe8c83512321f0e5d950"
result: "pass"
captured_at: "2026-10-23T00:06:06.591721881+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy -p astromech-intent -p astromech-tools -p astromech-exploration -p astromech-history -p astromech-core --all-targets -- -D warnings && python3 scripts/check-deps.py && python3 scripts/check-guide.py && cargo test --workspace'"
exit_status: "0"
agent_identity: "claude-implementer-m3xd"
target:
  kind: "issue"
  id: "astromech-m3xd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-m3xd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Gates: cargo fmt --check, clippy -D warnings on touched crates, check-deps, check-guide, cargo test --workspace (1138 passed, 0 failed)"
updated_at: "2026-10-23T00:06:39.437648948+00:00"
---

## Summary

Gates: cargo fmt --check, clippy -D warnings on touched crates, check-deps, check-guide, cargo test --workspace (1138 passed, 0 failed)

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy -p astromech-intent -p astromech-tools -p astromech-exploration -p astromech-history -p astromech-core --all-targets -- -D warnings && python3 scripts/check-deps.py && python3 scripts/check-guide.py && cargo test --workspace'
```

Exit status: 0

## Stdout

Bytes: 85578
Truncated: yes

```text
dependency rules OK (29 workspace crates)
checked 22 guide pages: 0 problem(s)

running 8 tests
test known_bodies::tests::lines ... ok
test session_summary::tests::credits_round_into_millions_at_the_boundary ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test catalog_tests::every_built_in_rule_has_a_title_and_a_default ... ok
test tests::settings_read_from_toml ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test route_follow::tests::the_toggle_turns_following_off_but_not_where_you_are ... ok
test route_follow::tests::only_live_arrivals_move_the_route ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
test how_long_ago_reads_in_the_nearest_unit ... ok
test a_new_system_says_nothing_about_visits ... ok
test a_system_visited_twice_before_says_so_and_when_last ... ok
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test landings_lines ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test a_body_landed_on_before_is_named_with_how_often ... ok
test the_visits_line_follows_the_state_line_within_the_word_budget ... ok
test several_bodies_are_counted_and_none_says_nothing ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok
test the_visits_line_is_silent_in_the_catch_up_and_without_a_history ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test can_be_turned_off ... ok
test a_late_answer_awaited_in_one_system_is_not_said_in_the_next ... ok
test a_relog_into_a_system_already_scanned_says_nothing ... ok
test values_and_lines_read_well ... ok
test arrival_with_known_biology_says_it_before_any_scan ... ok
test a_late_answer_is_said_on_the_tick_within_a_minute_and_once ... ok
test a_carrier_jump_or_a_login_is_an_arrival_too ... ok
test the_summary_follows_the_fss_and_the_dss ... ok
test each_system_starts_afresh ... ok

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
test fuel_callouts_are_silent_during_the_catch_up ... ok
test enough_fuel_gives_no_warning ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test hge_callouts_are_silent_during_the_catch_up ... ok
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test other_signals_stay_quiet ... ok
test a_burst_of_hges_from_one_honk_is_said
```

## Stderr

Bytes: 10289
Truncated: yes

```text
    Checking astromech-core v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-m3xd/crates/astromech-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.67s
warning: output filename collision at /home/supadava/projects/astromech-wt/astromech-m3xd/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-m3xd/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-m3xd/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt/astromech-m3xd/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-m3xd/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-m3xd/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-77a433d568d00918)
     Running tests/arrival.rs (target/debug/deps/arrival-a6aceba3886e096b)
     Running tests/biology.rs (target/debug/deps/biology-76878795e30c9354)
     Running tests/discovery.rs (target/debug/deps/discovery-97074e145139a9f6)
     Running tests/docking.rs (target/debug/deps/docking-da66a64725a42fa1)
     Running tests/fuel.rs (target/debug/deps/fuel-4f3d084b0f99a167)
     Running tests/hge.rs (target/debug/deps/hge-a32d75d9b7d663e2)
     Running tests/materials_cap.rs (target/debug/deps/materials_cap-43b55b5705f1d612)
     Running tests/missions.rs (target/debug/deps/missions-a8d36da2257b84cb)
     Running tests/powerplay_arrival.rs (target/debug/deps/powerplay_arrival-26da50c3bf2ffff8)
     Running tests/project_market.rs (target/debug/deps/project_market-e43dfd31efc5c98c)
     Running tests/prospector.rs (target/debug/deps/prospector-ce966b245c6fd680)
     Running tests/replay.rs (target/debug/deps/replay-1a9f08578542beaf)
     Running tests/route.rs (target/debug/deps/route-a07a7be08cee8156)
     Running tests/sampling.rs (target/debug/deps/sampling-634f7e0d2b50e52a)
     Running tests/scan.rs (target/debug/deps/scan-a51c17bc3a5f530a)
     Running tests/session_summary.rs (target/debug/deps/session_summary-4b29ede95b74c936)
     Running tests/signals.rs (target/debug/deps/signals-3166368aeaf96b75)
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-c6b827c46004ecec)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-2a3fd23e4a972a03)
     Running unittests src/main.rs (target/debug/deps/astromech_core-55ff3eb16bf0b724)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-34a8a89f7bc52ef3)
     Running tests/copy.rs (target/debug/deps/copy-87a0a76b051bd61c)
     Running tests/exploration_voice.rs (target/debug/deps/exploration_voice-c659c4bc210b9131)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-6c20304a0813c212)
     Running tests/gear_down.rs (target/debug/deps/gear_down-521674add9dfcd34)
     Running tests/known_bodies.rs (target/debug/deps/known_bodies-67f920181831d71f)
     Running tests/mining_status.rs (target/debug/deps/mining_status-b09a5d5c511a7118)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-f2241945643267bd)
     Running tests/real_journals.rs (target/debug/deps/real_journals-ef5e3ee
```

