---
created_at: "2026-10-16T20:42:26.265937266+00:00"
id: "astromech-sij2"
evidence_type: "test"
commit: "1e754bd9cb42a5076ca1ce7cdc7abbfe1600b0a6"
result: "pass"
captured_at: "2026-10-16T20:41:49.859486747+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm test && pnpm build'"
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
title: "After merging main (#24): workspace tests incl. speech (busy through render and after renderer panic, set_voice, replies before warm-ups, preview slot), callouts, copy guide; clippy -D warnings; dependency rules; shell lock; app install/lint/check/test (54)/build"
updated_at: "2026-10-16T20:42:26.352477693+00:00"
---

## Summary

After merging main (#24): workspace tests incl. speech (busy through render and after renderer panic, set_voice, replies before warm-ups, preview slot), callouts, copy guide; clippy -D warnings; dependency rules; shell lock; app install/lint/check/test (54)/build

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm test && pnpm build'
```

Exit status: 0

## Stdout

Bytes: 27365
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_rock_below_threshold_gets_nothing ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_motherlode_always_gets_a_line ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 8 tests
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test bad_settings_are_reported_not_fatal ... ok
test a_disabled_rule_never_fires ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok
test each_scan_type_gives_its_line ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 68 tests
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test crash::tests::marker_round_trip ... ok
test calibration::tests::steps_and_errors ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test calibration::tests::level_check_verdicts ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test control::tests::round_trip_and_token ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test setup::tests:
```

## Stderr

Bytes: 4690
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.71s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-6358391587029419)
     Running tests/prospector.rs (target/debug/deps/prospector-b7941a2252a31635)
     Running tests/replay.rs (target/debug/deps/replay-03a86a3675fb41a8)
     Running tests/scan.rs (target/debug/deps/scan-05993381616c1c5a)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-d12991ddc3a2d2ad)
     Running unittests src/main.rs (target/debug/deps/astromech_core-7e7108d7bf334384)
     Running tests/copy.rs (target/debug/deps/copy-b06c1cffe7d04ae3)
     Running tests/gear_down.rs (target/debug/deps/gear_down-7e205861c564aa2c)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-0894d0176ca77874)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-942fbfe589df60b0)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-4d6019b18242c6a6)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-acdcab3d59c11223)
     Running tests/companions.rs (target/debug/deps/companions-5573e4e769cee3cb)
     Running tests/mining.rs (target/debug/deps/mining-3eb3d02cb670261f)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-f4c6dcf9c8519f66)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-6dfddf9596e437bb)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-46b443a73d240c6b)
     Running unittests src/lib.rs (target
```

