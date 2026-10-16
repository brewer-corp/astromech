---
created_at: "2026-10-16T19:22:10.032664327+00:00"
id: "astromech-gyjh"
evidence_type: "test"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:22:02.800007508+00:00"
command: "bash -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py'"
updated_at: "2026-10-16T19:22:10.033106513+00:00"
---

## Summary

bash -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py'

## Command

```console
bash -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py'
```

Exit status: 0

## Stdout

Bytes: 13173
Truncated: yes

```text

running 28 tests
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::debug_input_logs_every_button ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::game_without_launcher ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::baked_public_key_parses ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test gamewatch::tests::launcher_game_exit ... ok
test update::tests::version_order ... ok
test logging::tests::rolls_over_and_prunes ... ok
test control::tests::round_trip_and_token ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test gamewatch::tests::lists_processes ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test silent_running_asks_then_runs ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test state_rules_press_nothing ... ok
test real_game_round_two ... ok
test did_you_mean_yes_learns_alias ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.52s


running 14 tests
test tests::trader_rates ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::fleet_from_history ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::catalogue_materials_resolve ... ok
test tests::planned_modules_follow_the_plan_like_the_game ... ok
test tests::trip_in_person_stops_and_remote_note ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 6 tests
test tests::here_from_journals ... ok
test tests::farming_notes_name_known_lookups ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test remote::tests::parse_and_match_fixtures ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test store::tests::seed_insert_cover_cache ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 1 test
test tests::folds_events ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test listen::model::tests::keys ... ok
test listen::b
```

## Stderr

Bytes: 5303
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
    Blocking waiting for file lock on package cache
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-z4jb/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-z4jb/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-z4jb/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-ship)` has the same output filename as the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-materials)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-z4jb/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-ship)` has the same output filename as the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-materials)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-c1a0e2887f5addfd)
     Running unittests src/main.rs (target/debug/deps/astromech_core-1b6f97b24778951c)
     Running tests/gear_down.rs (target/debug/deps/gear_down-e1388d22c9411275)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-47de08a4b04b1860)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-21a24efd2b8cda42)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-63ba36b87762ef71)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-8c30d105fc6d1015)
     Running tests/listen.rs (target/debug/deps/listen-60b2d3a5beaad99d)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-0aadfc2f5d71280c)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-514a54a502c408af)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-8416b38c54a0aceb)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-c73768c8bd01a1
```

