---
created_at: "2026-10-16T19:47:54.705399060+00:00"
id: "astromech-6iqf"
evidence_type: "test"
commit: "bdcef370136c610768e12ec0233e1b97f9f53b66"
result: "pass"
captured_at: "2026-10-16T19:47:49.260553233+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ngyt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ngyt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Workspace checks: clippy -D warnings (Linux; Windows-only WindowsNotifier not compiled), full workspace tests, dependency rules, shell Cargo.lock --locked"
updated_at: "2026-10-16T19:47:54.705639379+00:00"
---

## Summary

Workspace checks: clippy -D warnings (Linux; Windows-only WindowsNotifier not compiled), full workspace tests, dependency rules, shell Cargo.lock --locked

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 15495
Truncated: yes

```text

running 2 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test catch_up_leaves_docking_quiet_in_place ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test quiet_during_combat ... ok
test a_disabled_rule_never_fires ... ok
test quiet_during_docking ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 35 tests
test autostart::tests::command_line_quotes_the_path ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::bounce_gives_one_clip ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test speech::tests::cache_names_are_stable ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test logging::tests::rolls_over_and_prunes ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::version_order ... ok
test update::tests::copy_skips_data_dir ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test control::tests::round_trip_and_token ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test gamewatch::tests::lists_processes ... ok
test update::tests::signs_and_verifies_manifest ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test silent_running_asks_then_runs ... ok
test did_you_mean_then_yes_executes ... ok
test state_rules_press_nothing ... ok
test did_you_mean_yes_learns_alias ... ok
test real_game_round_two ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.52s


running 11 tests
test tests::trader_rates ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::fleet_from_history ... ok
test tests::catalogue_materials_resolve ... ok
```

## Stderr

Bytes: 4122
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-cmhv/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-cmhv/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-a3f66b342b676e5d)
     Running tests/replay.rs (target/debug/deps/replay-4d37a7fcb51f03e3)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-731694b3bdcc8737)
     Running unittests src/main.rs (target/debug/deps/astromech_core-37563b872a437932)
     Running tests/gear_down.rs (target/debug/deps/gear_down-bd859259b594cf4d)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-47de08a4b04b1860)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-21a24efd2b8cda42)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-551a46c45e29fe62)
     Running tests/companions.rs (target/debug/deps/companions-da38797e99658c4f)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-8c30d105fc6d1015)
     Running tests/listen.rs (target/debug/deps/listen-60b2d3a5beaad99d)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-0aadfc2f5d71280c)
     Running tests/events.rs (target/debug/deps/events-1b704c4477c61bee)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-514a54a502c408af)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-8416b38c54a0aceb)
     Running tests/live_journal.rs (target/debug/deps/live_journal-b2b1dd777f8a5d42)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-c73768c8bd01a11f)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-d7bc19546b5b1b63)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-f4c6dcf9c8519f66)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-6dfddf9596e437bb)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-27fbc016e30e0fd2)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-b98b6d50850e8a1e)
   Doc-tests astromech_announce
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
```

