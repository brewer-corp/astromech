---
created_at: "2026-10-22T21:12:33.343788263+00:00"
id: "astromech-vzlc"
evidence_type: "test"
commit: "746d65b46abf05e2dd695a5a1e2c57a9cf08c46c"
result: "pass"
captured_at: "2026-10-22T21:12:15.916218060+00:00"
command: "cargo test -p astromech-core -p astromech-data"
exit_status: "0"
agent_identity: "claude-implementer-g3qt"
target:
  kind: "issue"
  id: "astromech-nrhv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-nrhv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review R2 (astromech-ek06) fixed: the game's journal that can't be opened at startup takes the listing's length; pipeline test a_journal_that_cannot_be_opened_at_startup_is_still_a_replay_up_to_its_length sends only the appended line (3 before); all core + data tests pass"
updated_at: "2026-10-22T21:12:33.344184501+00:00"
---

## Summary

Review R2 (astromech-ek06) fixed: the game's journal that can't be opened at startup takes the listing's length; pipeline test a_journal_that_cannot_be_opened_at_startup_is_still_a_replay_up_to_its_length sends only the appended line (3 before); all core + data tests pass

## Command

```console
cargo test -p astromech-core -p astromech-data
```

Exit status: 0

## Stdout

Bytes: 21362
Truncated: yes

```text

running 173 tests
test app::tests::route_commands_over_the_control_api ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test app::tests::the_status_line_says_when_push_to_talk_is_only_testing ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout ... ok
test app::tests::the_sample_config_written_by_open_config_changes_no_setting ... ok
test app::tests::plotted_routes_load_into_the_follower ... ok
test background::tests::an_answer_is_looked_up_again_once_it_is_kept_for_long_enough ... ok
test app::tests::edsm_settings_keep_the_key_in_the_keyring_and_upload_only_with_both ... ok
test biology::tests::the_view_lists_bodies_known_and_sampling ... ok
test background::tests::a_failing_lookup_is_retried_once_per_back_off_while_a_callout_waits ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test biology::tests::a_recorded_answer_becomes_the_known_bodies ... ok
test calibration::tests::steps_and_errors ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test calibration::tests::level_check_verdicts ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test app::tests::setup_and_calibration_reply_shapes_match_the_window_fake ... ok
test biology::tests::the_first_ask_starts_a_lookup_and_later_asks_answer ... ok
test callouts::tests::the_shopping_list_is_kept_per_ship_and_merged ... ok
test callouts::tests::thresholds_open_on_a_fresh_machine_and_a_broken_store_is_logged ... ok
test app::tests::callout_settings_round_trip_through_the_config_file ... ok
test app::tests::fixing_a_broken_config_clears_the_error_without_a_restart ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test colonisation::tests::the_projects_are_replayed_when_the_journal_or_cargo_json_change ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::one_bad_value_costs_only_that_value ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::paste_button_fires_once_per_press ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::release_is_reported_when_the_debounce_window_ends ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::the_wait_is_the_idle_tick_except_to_end_a_debounce_window ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test eval::tests::empty_synthetic_transcripts_count_as_misses ... ok
test eval::tests::phrase_ids_count_per_command ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test gamewatch::tests::lists_processes ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test app::tests::a_journal_folder_set_after_startup_is_imported ... ok
test app::tests::set_eddn_flips_the_shared_switch_and_switching_off_empties_the_queue_at_once ... ok
test app::tests::fixing_ignored_values_updates_then_clears_the_note_without_a_restart ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test known_bodies::tests::a_recorded_answer_is_bodies_against_the_count ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test learned::tests::learned_phrases_are_listed_and_forgotten ... ok
test
```

## Stderr

Bytes: 1586
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-badbc78a43d6af96)
     Running unittests src/main.rs (target/debug/deps/astromech_core-c9de136d233e8c22)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-1ff76a2b3d92d0e5)
     Running tests/copy.rs (target/debug/deps/copy-421be2554c14c34c)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-9495851a457d687e)
     Running tests/gear_down.rs (target/debug/deps/gear_down-b7c29ff3a961deea)
     Running tests/known_bodies.rs (target/debug/deps/known_bodies-3521113958d32dec)
     Running tests/mining_status.rs (target/debug/deps/mining_status-32675eaf69fabbcb)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-03ddc2c4892e224f)
     Running tests/real_journals.rs (target/debug/deps/real_journals-1ca14bdfa999e998)
     Running tests/session_summary.rs (target/debug/deps/session_summary-db7506fcd47d760c)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-2ea4f04fb4b628fa)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-7774601e61d70683)
     Running tests/voice_eval_phrases.rs (target/debug/deps/voice_eval_phrases-e50bf97d6f10ccbd)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-072c4928a33e14ae)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-3bf74c347944c1dc)
     Running tests/recorded.rs (target/debug/deps/recorded-410c69bc3080a30b)
   Doc-tests astromech_core
   Doc-tests astromech_data
```

