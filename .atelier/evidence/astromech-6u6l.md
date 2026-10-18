---
created_at: "2026-10-17T23:21:11.547988763+00:00"
id: "astromech-6u6l"
evidence_type: "test"
commit: "6573fe176879354866b0e65c3d7efbd248a4f861"
result: "pass"
captured_at: "2026-10-17T23:20:54.242899591+00:00"
command: "cargo test -p astromech-core"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-r3s7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-r3s7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core"
updated_at: "2026-10-17T23:21:11.548335610+00:00"
---

## Summary

cargo test -p astromech-core

## Command

```console
cargo test -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 18304
Truncated: yes

```text

running 164 tests
test app::tests::a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout ... ok
test app::tests::route_commands_over_the_control_api ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test app::tests::the_status_line_says_when_push_to_talk_is_only_testing ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::the_sample_config_written_by_open_config_changes_no_setting ... ok
test app::tests::plotted_routes_load_into_the_follower ... ok
test biology::tests::the_view_lists_bodies_known_and_sampling ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test biology::tests::a_recorded_answer_becomes_the_known_bodies ... ok
test calibration::tests::steps_and_errors ... ok
test calibration::tests::level_check_verdicts ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test biology::tests::the_first_ask_starts_a_lookup_and_later_asks_answer ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test callouts::tests::the_shopping_list_is_kept_per_ship_and_merged ... ok
test callouts::tests::thresholds_open_on_a_fresh_machine_and_a_broken_store_is_logged ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test app::tests::a_journal_folder_set_after_startup_is_imported ... ok
test colonisation::tests::the_projects_are_replayed_when_the_journal_or_cargo_json_change ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test app::tests::callout_settings_round_trip_through_the_config_file ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::one_bad_value_costs_only_that_value ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test app::tests::fixing_ignored_values_updates_then_clears_the_note_without_a_restart ... ok
test controls::tests::paste_button_fires_once_per_press ... ok
test control::tests::round_trip_and_token ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test app::tests::set_eddn_flips_the_shared_switch_and_switching_off_empties_the_queue_at_once ... ok
test eval::tests::phrase_ids_count_per_command ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test eval::tests::empty_synthetic_transcripts_count_as_misses ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test gamewatch::tests::lists_processes ... ok
test app::tests::setup_and_calibration_reply_shapes_match_the_window_fake ... ok
test app::tests::fixing_a_broken_config_clears_the_error_without_a_restart ... ok
test app::tests::edsm_settings_keep_the_key_in_the_keyring_and_upload_only_with_both ... ok
test known_bodies::tests::a_recorded_answer_is_bodies_against_the_count ... ok
test learned::tests::learned_phrases_are_listed_and_forgotten ... ok
test lookups::tests::a_system_spansh_doesnt_know_is_searched_from_its_position_or_said_plainly ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test eval::tests::groups_by_family_and_condition_and_lists_misses ... ok
test callouts::tests::waits_while_the_speaker_is_b
```

## Stderr

Bytes: 1395
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-6845259f087e785f)
     Running unittests src/main.rs (target/debug/deps/astromech_core-c2505c6f352163c7)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-831af2eb1815f810)
     Running tests/copy.rs (target/debug/deps/copy-bfbe7d99ca39b730)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-3ce950859e799527)
     Running tests/gear_down.rs (target/debug/deps/gear_down-26d578ac3418192b)
     Running tests/known_bodies.rs (target/debug/deps/known_bodies-9c7d04f7651d636d)
     Running tests/mining_status.rs (target/debug/deps/mining_status-d48e42086a0548d5)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-9318045afcf5defb)
     Running tests/real_journals.rs (target/debug/deps/real_journals-f4ffaceedadef627)
     Running tests/session_summary.rs (target/debug/deps/session_summary-83b6a1dc4b5d6ea4)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-c8da83469f157457)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-81bb34262bba9828)
     Running tests/voice_eval_phrases.rs (target/debug/deps/voice_eval_phrases-bd5310dd93395695)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-b1a43e19b5f88e5c)
   Doc-tests astromech_core
```

