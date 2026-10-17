---
created_at: "2026-10-16T22:32:49.869085991+00:00"
id: "astromech-up70"
evidence_type: "test"
commit: "9c7644efaeb9c635562bc7a1c61c5ff225b7d2a1"
result: "pass"
captured_at: "2026-10-16T22:32:21.929601332+00:00"
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
title: "After merging main (#28 EDDN): workspace tests incl. pipeline (callout catch-up boundary, older journal re-read as replay fires nothing, delivery outside the stats lock, EDDN live/replay feed), callout replay tests, EDDN crate; clippy -D warnings; dependency rules; shell lock; app install/lint/check/test (82)/build"
updated_at: "2026-10-16T22:32:49.869382974+00:00"
---

## Summary

After merging main (#28 EDDN): workspace tests incl. pipeline (callout catch-up boundary, older journal re-read as replay fires nothing, delivery outside the stats lock, EDDN live/replay feed), callout replay tests, EDDN crate; clippy -D warnings; dependency rules; shell lock; app install/lint/check/test (82)/build

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm test && pnpm build'
```

Exit status: 0

## Stdout

Bytes: 41564
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_rock_above_threshold_gets_one_short_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_motherlode_always_gets_a_line ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test a_disabled_rule_never_fires ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test bad_settings_are_reported_not_fatal ... ok
test quiet_during_docking ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test quiet_during_combat ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok
test each_scan_type_gives_its_line ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 75 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::state_round_trip ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test calibration::tests::steps_and_errors ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test crash::tests::marker_round_trip ... ok
test control::tests::round_trip_and_token ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::launcher_game_exit ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test calibration::tests::level_check_verdicts ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test logging::tests::rolls_over_and_prunes ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test gamewatch::tests::lists_processes ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test setup
```

## Stderr

Bytes: 5470
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-31481f111e81517f)
     Running tests/prospector.rs (target/debug/deps/prospector-af6d2cafc8a288de)
     Running tests/replay.rs (target/debug/deps/replay-740b020327aad20c)
     Running tests/scan.rs (target/debug/deps/scan-2761c5e7ca0eebd3)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-eba63e675e3d85ae)
     Running unittests src/main.rs (target/debug/deps/astromech_core-5deff39db813310c)
     Running tests/copy.rs (target/debug/deps/copy-d6a48d6b76b79b96)
     Running tests/gear_down.rs (target/debug/deps/gear_down-d5775c1349b5dd28)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-676c7ff4707ed6c7)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-bf95005bdf3019c6)
     Running tests/recorded.rs (target/debug/deps/recorded-80d516da36ee0b80)
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-9e9c786a955c74be)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-9853607d0eb0b74b)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-179b273388736436)
     Running tests/powerplay.rs (target/debug/deps/powerplay-fbc56d67f8f2ccf1)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-3b556f8b77638dff)
     Running tests/companions.rs (target/debug/deps/companions-ff2622d94306b493)
     Running tests/mining.rs (target/debug/deps/mining-8554475f4168a141)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-12c154904ddbf506)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-96c9823824005daa)
     Running tests/listen.rs (target/debug/deps/listen-471f8114a4c1a654)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-d6676a1c5bf63860)
     Running tests/events.rs (target/debug/deps/events-d0979b58cfde7e67)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-cf12baa3e1fdee4f)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-fb07dc04605130f2)
     Running tests/live_journal.rs (target/debug/deps/live_journal-2693deaab3ba4288)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-9e393abaef84edea)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-b27679dce68d9b88)
     Running unittests src/lib.rs (target/debug/deps/astromech_powerplay-df1165627369c35f)
     Running tests/arrival.rs (target/debug/d
```

