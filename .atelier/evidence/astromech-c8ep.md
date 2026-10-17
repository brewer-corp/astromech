---
created_at: "2026-10-17T18:39:58.655241715+00:00"
id: "astromech-c8ep"
evidence_type: "test"
commit: "f02249dcc2ffa2c48d26a5dc68d206b38a4a94ee"
result: "pass"
captured_at: "2026-10-17T18:39:43.675780566+00:00"
command: "cargo test -p astromech-journal -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-p4um"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-p4um"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-journal -p astromech-core"
updated_at: "2026-10-17T18:39:58.655635971+00:00"
---

## Summary

cargo test -p astromech-journal -p astromech-core

## Command

```console
cargo test -p astromech-journal -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 16397
Truncated: yes

```text

running 121 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test app::tests::route_commands_over_the_control_api ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test calibration::tests::level_check_verdicts ... ok
test calibration::tests::steps_and_errors ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test biology::tests::the_view_lists_bodies_known_and_sampling ... ok
test callouts::tests::the_shopping_list_is_kept_per_ship_and_merged ... ok
test biology::tests::a_recorded_answer_becomes_the_known_bodies ... ok
test colonisation::tests::the_projects_are_replayed_when_the_journal_or_cargo_json_change ... ok
test biology::tests::the_first_ask_starts_a_lookup_and_later_asks_answer ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::paste_button_fires_once_per_press ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test gamewatch::tests::lists_processes ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test app::tests::a_journal_folder_set_after_startup_is_imported ... ok
test app::tests::edsm_settings_keep_the_key_in_the_keyring_and_upload_only_with_both ... ok
test lookups::tests::a_system_spansh_doesnt_know_is_searched_from_its_position_or_said_plainly ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test app::tests::set_eddn_flips_the_shared_switch_and_switching_off_empties_the_queue_at_once ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test lookups::tests::the_faction_and_powerplay_lookups_share_one_no_position_line ... ok
test pipeline::tests::a_line_appended_to_the_open_journal_is_applied_when_idle ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_on_the_next_status_read ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_when_idle ... ok
test pipeline::tests::a_pending_event_whose_file_never_comes_is_given_up_once ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test pipeline::tests::a_retry_never_runs_ahead_of_the_journal_lines_of_its_batch ... ok
test pipeline::tests::an_idle_pipeline_follows_a_new_journal_within_a_second ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::backpack_json_is_read_again_after_a_backpack_change ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::eddn_get
```

## Stderr

Bytes: 1400
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
     Running unittests src/main.rs (target/debug/deps/astromech_core-9fd34ab1155492f0)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-7dbee4a6e5ae421b)
     Running tests/copy.rs (target/debug/deps/copy-a57b61b54ffa7e99)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-af66098fe9faae44)
     Running tests/gear_down.rs (target/debug/deps/gear_down-5ef75a64e503c151)
     Running tests/mining_status.rs (target/debug/deps/mining_status-716eb4759885c4d8)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-824fcf3718590439)
     Running tests/session_summary.rs (target/debug/deps/session_summary-2e1f3729458e5b72)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-8b50d1a4331f3a34)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-b49cf8fc39e39794)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-ead47f3575d86951)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-69ac2b430a44a413)
     Running tests/events.rs (target/debug/deps/events-e87652f8b9a6d75a)
     Running tests/live_watch.rs (target/debug/deps/live_watch-1ae637a337a2134d)
   Doc-tests astromech_core
   Doc-tests astromech_journal
```

