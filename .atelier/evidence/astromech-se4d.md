---
created_at: "2026-10-16T20:32:14.497340004+00:00"
id: "astromech-se4d"
evidence_type: "test"
commit: "59d95e73c211aab2830d0dfbd765f99fab2de544"
result: "pass"
captured_at: "2026-10-16T20:31:41.009077723+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm test && pnpm build'"
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
title: "After callout review fixes R1/R5/R7/R8/R10: scan replay test (crime then stolen cargo during docking speaks the urgent line), depleted-rock test, copy test over rule ids, motherlode+two ores and unavailable replies; plus clippy -D warnings, workspace tests, dependency rules, shell lock, app lint/check/test/build"
updated_at: "2026-10-16T20:32:14.497738238+00:00"
---

## Summary

After callout review fixes R1/R5/R7/R8/R10: scan replay test (crime then stolen cargo during docking speaks the urgent line), depleted-rock test, copy test over rule ids, motherlode+two ores and unavailable replies; plus clippy -D warnings, workspace tests, dependency rules, shell lock, app lint/check/test/build

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm test && pnpm build'
```

Exit status: 0

## Stdout

Bytes: 26461
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
test a_disabled_rule_never_fires ... ok
test bad_settings_are_reported_not_fatal ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok
test each_scan_type_gives_its_line ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 47 tests
test autostart::tests::command_line_quotes_the_path ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test control::tests::text_argument_is_optional ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::game_without_launcher ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::launcher_game_exit ... ok
test gamewatch::tests::finds_newest_journal ... ok
test logging::tests::rolls_over_and_prunes ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test control::tests::round_trip_and_token ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test speech::tests::busy_while_a_line_renders_or_plays_without_a_device ... ok
test tester::tests::lease_expires ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::a_panicking_renderer_does_not_leave_the_speaker_busy ... ok
test update::tests::baked_public_key_parses ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::version_order ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test gamewatch::tests::lists_processes ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test tester::tests::plan_reflects_game_state ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test update::tests::signs_and_
```

## Stderr

Bytes: 4690
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
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
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-227d14bf08a5f3c1)
     Running tests/prospector.rs (target/debug/deps/prospector-454c068e0f0a25e7)
     Running tests/replay.rs (target/debug/deps/replay-385379d9602fbb34)
     Running tests/scan.rs (target/debug/deps/scan-a90b84bee48dec26)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-c11e1b576b623820)
     Running unittests src/main.rs (target/debug/deps/astromech_core-486976f0f896fdbc)
     Running tests/copy.rs (target/debug/deps/copy-0488253adc0646dd)
     Running tests/gear_down.rs (target/debug/deps/gear_down-a1bc40e45d727c9b)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-e5021df8b3f40314)
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

