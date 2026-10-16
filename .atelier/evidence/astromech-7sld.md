---
created_at: "2026-10-16T18:39:19.854261772+00:00"
id: "astromech-7sld"
evidence_type: "test"
commit: "47d9089873fe481e711b259b431b81eb160cd973"
result: "pass"
captured_at: "2026-10-16T18:39:14.440485096+00:00"
command: "cargo test -p astromech-gamestate -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-9uqb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-9uqb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Replay of a journal fixture against fixture companion files leaves each snapshot in game state; missing, half-written and stale files are reported and not used"
updated_at: "2026-10-16T18:39:19.854688388+00:00"
---

## Summary

Replay of a journal fixture against fixture companion files leaves each snapshot in game state; missing, half-written and stale files are reported and not used

## Command

```console
cargo test -p astromech-gamestate -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 3659
Truncated: no

```text

running 30 tests
test autostart::tests::command_line_quotes_the_path ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test autostart::tests::state_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test config::tests::finds_look_alike_config_names ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::copy_skips_data_dir ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test update::tests::version_order ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test control::tests::round_trip_and_token ... ok
test gamewatch::tests::lists_processes ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


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

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.55s


running 1 test
test tests::folds_events ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test events_map_to_files ... ok
test a_file_older_than_its_event_is_reported_and_not_used ... ok
test a_missing_file_is_reported ... ok
test only_the_latest_event_of_a_kind_is_checked ... ok
test each_event_leaves_its_snapshot_in_game_state ... ok
test a_half_written_file_is_reported ... ok
test a_stale_file_does_not_replace_the_snapshot_already_held ... ok
test a_file_the_game_writes_a_moment_late_is_picked_up_by_the_retry ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 677
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tpdb/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.29s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3ef5b9319d242e60)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8db4e83b3fbc02c2)
     Running tests/gear_down.rs (target/debug/deps/gear_down-940437f5cc7f0d85)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-551a46c45e29fe62)
     Running tests/companions.rs (target/debug/deps/companions-da38797e99658c4f)
   Doc-tests astromech_core
   Doc-tests astromech_gamestate
```

