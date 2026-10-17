---
created_at: "2026-10-16T19:47:44.641835547+00:00"
id: "astromech-m0h4"
evidence_type: "test"
commit: "bdcef370136c610768e12ec0233e1b97f9f53b66"
result: "pass"
captured_at: "2026-10-16T19:47:37.783715064+00:00"
command: "cargo test -p astromech-announce -p astromech-core"
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
title: "Replay tests: a rule fires once per event within its cooldown; a disabled rule never fires; nothing fires during the startup journal catch-up (engine and real pipeline Tailer, events before and after the live point); quiet during docking and combat; speech queue order"
updated_at: "2026-10-16T19:47:44.642104219+00:00"
---

## Summary

Replay tests: a rule fires once per event within its cooldown; a disabled rule never fires; nothing fires during the startup journal catch-up (engine and real pipeline Tailer, events before and after the live point); quiet during docking and combat; speech queue order

## Command

```console
cargo test -p astromech-announce -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 3960
Truncated: no

```text

running 2 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test catch_up_leaves_docking_quiet_in_place ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test a_disabled_rule_never_fires ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 35 tests
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test controls::tests::debug_input_logs_every_button ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::launcher_game_exit ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test logging::tests::rolls_over_and_prunes ... ok
test control::tests::round_trip_and_token ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test speech::tests::cache_names_are_stable ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test update::tests::version_order ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::copy_skips_data_dir ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test silent_running_asks_then_runs ... ok
test status_questions_are_spoken_not_pressed ... ok
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


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 667
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-a3f66b342b676e5d)
     Running tests/replay.rs (target/debug/deps/replay-4d37a7fcb51f03e3)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-2d09ffff36ad49af)
     Running unittests src/main.rs (target/debug/deps/astromech_core-62fa82cac76523f5)
     Running tests/gear_down.rs (target/debug/deps/gear_down-2ec8f3822225da7c)
   Doc-tests astromech_announce
   Doc-tests astromech_core
```

