---
created_at: "2026-10-16T21:56:21.932019219+00:00"
id: "astromech-qyba"
evidence_type: "check"
commit: "b73a94bf7f28e36b03839557d3a216747c6fe910"
result: "pass"
captured_at: "2026-10-16T21:56:11.416670413+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --format-version 1 >/dev/null'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-m30a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-m30a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main: fmt, clippy -D warnings, workspace tests, check-deps, shell Cargo.lock current"
updated_at: "2026-10-16T21:56:21.932477363+00:00"
---

## Summary

After merging main: fmt, clippy -D warnings, workspace tests, check-deps, shell Cargo.lock current

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --format-version 1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 23359
Truncated: yes

```text

running 65 tests
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::bounce_gives_one_clip ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::debug_input_logs_every_button ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test gamewatch::tests::finds_newest_journal ... ok
test calibration::tests::steps_and_errors ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test calibration::tests::level_check_verdicts ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test control::tests::round_trip_and_token ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test gamewatch::tests::lists_processes ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::cache_names_are_stable ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test speech::tests::preview_slot_lifecycle ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test tester::tests::lease_expires ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test tester::tests::plan_reflects_game_state ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::copy_skips_data_dir ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test update::tests::version_order ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test update::tests::signs_and_verifies_manifest ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test tester::tests::lookups_say_what_they_would_look_up ... ok
test history::tests::follows_appends_without_notifications ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok
test lookups::tests::ship_phrases_resolve_to_lookups ... ok
test lookups::tests::module_phrases_resolve_to_lookups ... ok
test lookups::tests::service_phrases_resolve_to_lookups ... ok

test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.75s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_ke
```

## Stderr

Bytes: 4566
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
    Blocking waiting for file lock on package cache
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-m30a/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-m30a/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-866ac3af8c337b7e)
     Running unittests src/main.rs (target/debug/deps/astromech_core-7fdcb761c8659a63)
     Running tests/gear_down.rs (target/debug/deps/gear_down-e5dd7760d9542afd)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-83e54570f569cd5e)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-942fbfe589df60b0)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-4d6019b18242c6a6)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-551a46c45e29fe62)
     Running tests/companions.rs (target/debug/deps/companions-da38797e99658c4f)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-b5dac7a6fec64879)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-84e0e81be2efc563)
     Running tests/coriolis.rs (target/debug/deps/coriolis-670c639f19b2ffcc)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-638b826418a49c64)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-764f9010b8c5d158)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-807f84173a65359d)
   Doc-tests astromech_core
   Doc-tests astromech_data
   Doc-tests astromech_engineering
   Doc-tests a
```

