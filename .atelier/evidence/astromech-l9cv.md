---
created_at: "2026-10-16T23:14:41.920935269+00:00"
id: "astromech-l9cv"
evidence_type: "test"
commit: "b01504470817bdc903d8af8df2756035389e8f90"
result: "pass"
captured_at: "2026-10-16T23:14:29.219755454+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-cmhv"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-cmhv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Windows CI fix for callouts_are_silent_for_the_startup_catch_up_and_live_after_it (older-journal touch via closed handle + explicit mtime); Linux: pipeline tests pass, and fail with the replay boundary removed; plus clippy -D warnings, workspace tests, dependency rules, shell lock. Windows rerun pending CI."
updated_at: "2026-10-16T23:14:41.921300833+00:00"
---

## Summary

Windows CI fix for callouts_are_silent_for_the_startup_catch_up_and_live_after_it (older-journal touch via closed handle + explicit mtime); Linux: pipeline tests pass, and fail with the replay boundary removed; plus clippy -D warnings, workspace tests, dependency rules, shell lock. Windows rerun pending CI.

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 36947
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test a_motherlode_always_gets_a_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test threshold_changes_from_settings_apply_at_once ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test a_disabled_rule_never_fires ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test quiet_during_combat ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_docking ... ok
test catch_up_leaves_docking_quiet_in_place ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok
test each_scan_type_gives_its_line ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 76 tests
test autostart::tests::state_round_trip ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::steps_and_errors ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test calibration::tests::level_check_verdicts ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test control::tests::round_trip_and_token ... ok
test crash::tests::marker_round_trip ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test gamewatch::tests::finds_newest_journal ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test logging::tests::rolls_over_and_prunes ... ok
test gamewatch::tests::lists_processes ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::t
```

## Stderr

Bytes: 5937
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-cmhv/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-cmhv/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-0bd349a8d045c7fa)
     Running tests/prospector.rs (target/debug/deps/prospector-2c265afbf4ea3e1b)
     Running tests/replay.rs (target/debug/deps/replay-d65a1bd159f2d82c)
     Running tests/scan.rs (target/debug/deps/scan-573568d140cb84ea)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-820f9b87f2ab74c9)
     Running unittests src/main.rs (target/debug/deps/astromech_core-ee00fe8a2faf5784)
     Running tests/copy.rs (target/debug/deps/copy-ad16e848cbc3f2f6)
     Running tests/gear_down.rs (target/debug/deps/gear_down-fac0a64a4cdf3e59)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-477a1af3c50acca1)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-4cdd6bb593accff4)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-6ed659280f0eb760)
     Running tests/recorded.rs (target/debug/deps/recorded-077bd2d8d758312c)
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-943ca53e72b74208)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ccdb1e86be63f36e)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-5811f7cd0f3a0534)
     Running tests/powerplay.rs (target/debug/deps/powerplay-7be4710b206f5244)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-5296df5f07f4fb66)
     Running tests/companions.rs (target/debug/deps/companions-47219eed008c5014)
     Running tests/mining.rs (target/debug/deps/mining-24f0a0d64f8ca403)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-24c9bd3f58aacc1d)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-71818ae170a2b90a)
     Running tests/listen.rs (target/debug/deps/listen-aefcf94f21ae37fb)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-49143e7857eb60de)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-45c2bd546db5006a)
     Running tests/events.rs (target/debug/deps/events-71fc1ec250dbbd8f)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-7a9ff47410104a2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_market-fb807e578ef26c19)
     Running tests/recorded.rs (target/debug/deps/recorded-1d149e13c60099e7)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-0267425cef0ef5e8)
     Running tests/live_journal.rs (target/debug/deps/live_journal-a4363f75cba5e1c5)
     Running unittests src/lib.rs (target/debug/deps/astromech_ov
```

