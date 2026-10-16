---
created_at: "2026-10-16T19:25:15.340368316+00:00"
id: "astromech-7qc6"
evidence_type: "test"
commit: "cbcfc2475678431decf542e4d0775b99a895675e"
result: "pass"
captured_at: "2026-10-16T19:25:07.233937311+00:00"
command: "cargo test --workspace"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ny3y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ny3y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace tests after the review fixes (calibration and setup included)"
updated_at: "2026-10-16T19:25:15.340608234+00:00"
---

## Summary

Workspace tests after the review fixes (calibration and setup included)

## Command

```console
cargo test --workspace
```

Exit status: 0

## Stdout

Bytes: 12699
Truncated: yes

```text

running 41 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::level_check_verdicts ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test gamewatch::tests::finds_newest_journal ... ok
test autostart::tests::state_round_trip ... ok
test controls::tests::bounce_gives_one_clip ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test config::tests::finds_look_alike_config_names ... ok
test control::tests::round_trip_and_token ... ok
test logging::tests::rolls_over_and_prunes ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test speech::tests::cache_names_are_stable ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test update::tests::version_order ... ok
test gamewatch::tests::lists_processes ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test setup::tests::onboarding_flag ... ok
test calibration::tests::steps_and_errors ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test update::tests::signs_and_verifies_manifest ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok

test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test silent_running_asks_then_runs ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test state_rules_press_nothing ... ok
test did_you_mean_then_yes_executes ... ok
test did_you_mean_yes_learns_alias ... ok
test real_game_round_two ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.57s


running 11 tests
test tests::trader_rates ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::fleet_from_history ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::catalogue_materials_resolve ... ok
test tests::trip_in_person_stops_and_remote_note ... ok
test tests::experimental_never_before_its_blueprint ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 6 tests
test tests::here_from_j
```

## Stderr

Bytes: 3616
Truncated: no

```text
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-lxma/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-lxma/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-82da8af47bb9cda9)
     Running unittests src/main.rs (target/debug/deps/astromech_core-174d16b324d0a4d9)
     Running tests/gear_down.rs (target/debug/deps/gear_down-97d3433906393810)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-47de08a4b04b1860)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-21a24efd2b8cda42)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-63ba36b87762ef71)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-8c30d105fc6d1015)
     Running tests/listen.rs (target/debug/deps/listen-60b2d3a5beaad99d)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-0aadfc2f5d71280c)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-514a54a502c408af)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-8416b38c54a0aceb)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-c73768c8bd01a11f)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-d7bc19546b5b1b63)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-f4c6dcf9c8519f66)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-6dfddf9596e437bb)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-27fbc016e30e0fd2)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-b98b6d50850e8a1e)
   Doc-tests astromech_core
   Doc-tests astromech_engineering
   Doc-tests astromech_galaxy
   Doc-tests astromech_gamestate
   Doc-tests astromech_input
   Doc-tests astromech_intent
   Doc-tests astromech_journal
   Doc-tests astromech_keybinds
   Doc-tests astromech_materials
   Doc-tests astromech_overlay
   Doc-tests astromech_policy
   Doc-tests astromech_proto
   Doc-tests astromech_status
   Doc-tests astromech_store
   Doc-tests astromech_tools
   Doc-tests astromech_voice
```

