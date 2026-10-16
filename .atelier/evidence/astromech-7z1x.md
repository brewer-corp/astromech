---
created_at: "2026-10-16T18:46:47.916452580+00:00"
id: "astromech-7z1x"
evidence_type: "test"
commit: "e71c2a37d3156520eaf2e8c0fb0b234c8c7fc24d"
result: "pass"
captured_at: "2026-10-16T18:46:43.780865085+00:00"
command: "cargo test -p astromech-journal -p astromech-gamestate -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-qgjp"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qgjp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Re-run after review fixes"
updated_at: "2026-10-16T18:46:47.916932877+00:00"
---

## Summary

Re-run after review fixes

## Command

```console
cargo test -p astromech-journal -p astromech-gamestate -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 5477
Truncated: yes

```text

running 30 tests
test autostart::tests::command_line_quotes_the_path ... ok
test crash::tests::marker_round_trip ... ok
test autostart::tests::state_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::bounce_gives_one_clip ... ok
test config::tests::finds_look_alike_config_names ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::launcher_game_exit ... ok
test logging::tests::rolls_over_and_prunes ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::debug_input_logs_every_button ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test update::tests::version_order ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test update::tests::baked_public_key_parses ... ok
test gamewatch::tests::game_without_launcher ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test gamewatch::tests::lists_processes ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test silent_running_asks_then_runs ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test state_rules_press_nothing ... ok
test did_you_mean_yes_learns_alias ... ok
test voice_path_with_fake_stt ... ok
test real_game_round_two ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.53s


running 1 test
test tests::folds_events ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test events_map_to_files ... ok
test a_missing_file_is_reported ... ok
test a_half_written_file_is_reported ... ok
test a_read_never_waits_for_a_file_the_game_has_not_written_yet ... ok
test a_file_older_than_its_event_is_reported_and_not_used ... ok
test only_the_latest_event_of_a_kind_is_checked ... ok
test station_market_id_follows_docking_login_and_undocking ... ok
test each_event_leaves_its_snapshot_in_game_state ... ok
test a_stale_file_does_not_replace_the_snapshot_already_held ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
test tests::cargo_and_route_files ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::known_and_unknown_events ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::loadout_modules_and_fleet_events ... ok
test tests::positions_and_engineer_events ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 28 tests
test cargo_inventory ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test exobiology_events ... ok
test bount
```

## Stderr

Bytes: 751
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3ef5b9319d242e60)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8db4e83b3fbc02c2)
     Running tests/gear_down.rs (target/debug/deps/gear_down-940437f5cc7f0d85)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-551a46c45e29fe62)
     Running tests/companions.rs (target/debug/deps/companions-da38797e99658c4f)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-0aadfc2f5d71280c)
     Running tests/events.rs (target/debug/deps/events-1b704c4477c61bee)
   Doc-tests astromech_core
   Doc-tests astromech_gamestate
   Doc-tests astromech_journal
```

