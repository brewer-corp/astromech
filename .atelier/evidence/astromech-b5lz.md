---
created_at: "2026-10-16T22:03:58.326309001+00:00"
id: "astromech-b5lz"
evidence_type: "test"
commit: "d7bf5a015d60fa7dd5f55b97aa0633e12fd671db"
result: "pass"
captured_at: "2026-10-16T22:03:28.882119017+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm test'"
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
title: "After merging main (#29 whole-line journal passing): workspace tests incl. pipeline (callout catch-up boundary, delivery outside the stats lock, companions) and callout replay tests; clippy -D warnings; dependency rules; shell lock; app install/test (79)"
updated_at: "2026-10-16T22:03:58.332072413+00:00"
---

## Summary

After merging main (#29 whole-line journal passing): workspace tests incl. pipeline (callout catch-up boundary, delivery outside the stats lock, companions) and callout replay tests; clippy -D warnings; dependency rules; shell lock; app install/test (79)

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 26469
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test a_motherlode_always_gets_a_line ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test threshold_changes_from_settings_apply_at_once ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test quiet_during_combat ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test bad_settings_are_reported_not_fatal ... ok
test a_disabled_rule_never_fires ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_docking ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
test each_scan_type_gives_its_line ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 71 tests
test autostart::tests::command_line_quotes_the_path ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test calibration::tests::level_check_verdicts ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test calibration::tests::steps_and_errors ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test config::tests::finds_look_alike_config_names ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::launcher_game_exit ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test logging::tests::rolls_over_and_prunes ... ok
test gamewatch::tests::lists_processes ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_
```

## Stderr

Bytes: 5356
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.51s
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-51ebd600ac8c6951)
     Running tests/prospector.rs (target/debug/deps/prospector-decbe5abdc06aef5)
     Running tests/replay.rs (target/debug/deps/replay-42f8b37d3499151e)
     Running tests/scan.rs (target/debug/deps/scan-2b62b2e61277009e)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-b4141b62652d1fc9)
     Running unittests src/main.rs (target/debug/deps/astromech_core-be41e3228a1ff186)
     Running tests/copy.rs (target/debug/deps/copy-4a076bb0490b098e)
     Running tests/gear_down.rs (target/debug/deps/gear_down-0bd514618f9eb835)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-d32f972ed76dae83)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-942fbfe589df60b0)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-b591a561b9f40aae)
     Running tests/powerplay.rs (target/debug/deps/powerplay-554269ec4e6822df)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-6a11f9ba2ca1c56e)
     Running tests/companions.rs (target/debug/deps/companions-f6fb8eab61a9afbd)
     Running tests/mining.rs (target/debug/deps/mining-bd9b4537f9fe4b29)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_powerplay-1daaf7d0369c5894)
     Running tests/arrival.rs (target/debug/deps/arrival-a9b54f786b2d034e)
     Running tests/standing.rs (target/debug/deps/standi
```

