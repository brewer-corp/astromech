---
created_at: "2026-10-17T00:03:52.846760850+00:00"
id: "astromech-sjma"
evidence_type: "test"
commit: "ea70d32458c19a631a8cc5aa70392e850ed30573"
result: "pass"
captured_at: "2026-10-17T00:03:39.221188876+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null)'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-5t34"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-5t34"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "workspace gates: fmt, clippy -D warnings, cargo test --workspace, check-deps, app metadata"
updated_at: "2026-10-17T00:03:52.847263829+00:00"
---

## Summary

workspace gates: fmt, clippy -D warnings, cargo test --workspace, check-deps, app metadata

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null)'
```

Exit status: 0

## Stdout

Bytes: 40228
Truncated: yes

```text

running 16 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::claims_and_beacons ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::shopping_tests::cargo_json_overrides_the_replayed_hold ... ok
test tests::save_writes_only_changes ... ok
test tests::survives_restart ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 73 tests
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::bounce_gives_one_clip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test calibration::tests::steps_and_errors ... ok
test calibration::tests::level_check_verdicts ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test logging::tests::rolls_over_and_prunes ... ok
test gamewatch::tests::lists_processes ... ok
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test app::tests::a_journal_folder_set_after_startup_is_imported ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::the_eddn_switch_is_saved_and_keeps_its_comment ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::c
```

## Stderr

Bytes: 7452
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
warning: output filename collision at /home/supadava/.cache/astromech-5t34/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/.cache/astromech-5t34/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-bug-astromech-5t34/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_colonisation-9ee2fa666954b1d4)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_core-4a711807a3431126)
     Running unittests src/main.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_core-65480abdb2a2c0da)
     Running tests/gear_down.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/gear_down-62923f5a2151aa96)
     Running tests/site_lookup.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/site_lookup-1386b0ac4037c54e)
     Running tests/trade_voice.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/trade_voice-0b3aa11d98c4d66f)
     Running tests/voice_tester.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/voice_tester-de59feedb25e7821)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_data-6ed659280f0eb760)
     Running tests/recorded.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/recorded-077bd2d8d758312c)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_eddn-943ca53e72b74208)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_engineering-ccdb1e86be63f36e)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_galaxy-5811f7cd0f3a0534)
     Running tests/powerplay.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/powerplay-279087c75a955c55)
     Running tests/sites.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/sites-61e89b44860c1b5a)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_gamestate-ed6a7f4851c962d5)
     Running tests/companions.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/companions-2addafdeb7d31f7a)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_history-24c9bd3f58aacc1d)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_input-71818ae170a2b90a)
     Running tests/listen.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/listen-aefcf94f21ae37fb)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_intent-49143e7857eb60de)
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-5t34/target/debug/deps/astromech_journal-45c2bd546db5006a)
     Running tests/events.rs (/home/supadava/.cache/astromech
```

