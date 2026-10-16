---
created_at: "2026-10-16T22:10:05.891377967+00:00"
id: "astromech-vy5t"
evidence_type: "test"
commit: "ce2de6e42a07dc41338407c665bad8cc5984b40a"
result: "pass"
captured_at: "2026-10-16T22:09:49.088385570+00:00"
command: "cargo test -p astromech-station -p astromech-journal -p astromech-core --locked"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-zig3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zig3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-station -p astromech-journal -p astromech-core --locked"
updated_at: "2026-10-16T22:10:05.891902093+00:00"
---

## Summary

cargo test -p astromech-station -p astromech-journal -p astromech-core --locked

## Command

```console
cargo test -p astromech-station -p astromech-journal -p astromech-core --locked
```

Exit status: 0

## Stdout

Bytes: 7794
Truncated: yes

```text

running 61 tests
test control::tests::text_argument_is_optional ... ok
test calibration::tests::level_check_verdicts ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::finds_look_alike_config_names ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::debug_input_logs_every_button ... ok
test gamewatch::tests::game_without_launcher ... ok
test controls::tests::bounce_gives_one_clip ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test gamewatch::tests::launcher_game_exit ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test control::tests::round_trip_and_token ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test gamewatch::tests::lists_processes ... ok
test calibration::tests::steps_and_errors ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::preview_slot_lifecycle ... ok
test setup::tests::onboarding_flag ... ok
test tester::tests::lease_expires ... ok
test update::tests::baked_public_key_parses ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test update::tests::copy_skips_data_dir ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test update::tests::version_order ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test tester::tests::plan_reflects_game_state ... ok
test update::tests::signs_and_verifies_manifest ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok

test result: ok. 61 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test silent_running_asks_then_runs ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test stat
```

## Stderr

Bytes: 1059
Truncated: no

```text
   Compiling astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-zig3/crates/astromech-station)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-zig3/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.89s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-e452f70fde1e800f)
     Running unittests src/main.rs (target/debug/deps/astromech_core-c05af9f9d7ce8a20)
     Running tests/gear_down.rs (target/debug/deps/gear_down-df8367cd66db5c92)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-89eb1a02f4fef731)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-0aadfc2f5d71280c)
     Running tests/events.rs (target/debug/deps/events-1b704c4477c61bee)
     Running unittests src/lib.rs (target/debug/deps/astromech_station-8af483bc79655d95)
     Running tests/dock.rs (target/debug/deps/dock-1ed4e91d001cb2a0)
   Doc-tests astromech_core
   Doc-tests astromech_journal
   Doc-tests astromech_station
```

