---
created_at: "2026-10-16T20:15:18.775980615+00:00"
id: "astromech-oub3"
evidence_type: "test"
commit: "7c78a26b408440227c35298c29ee224721bc0bec"
result: "pass"
captured_at: "2026-10-16T20:15:04.985119879+00:00"
command: "sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-0zcu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-0zcu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Scan replay tests: each scan type gives its line; stolen-goods suffix only with stolen cargo in the Cargo.json fixture (and only on cargo scans); urgent through docking quiet, once per 30 s; Cargo.json stolen parse; plus clippy -D warnings, workspace tests, dependency rules, shell lock"
updated_at: "2026-10-16T20:15:18.776546110+00:00"
---

## Summary

Scan replay tests: each scan type gives its line; stolen-goods suffix only with stolen cargo in the Cargo.json fixture (and only on cargo scans); urgent through docking quiet, once per 30 s; Cargo.json stolen parse; plus clippy -D warnings, workspace tests, dependency rules, shell lock

## Command

```console
sh -c 'cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 18261
Truncated: yes

```text

running 3 tests
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test a_rock_above_threshold_gets_one_short_line ... ok
test a_rock_below_threshold_gets_nothing ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_motherlode_always_gets_a_line ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
test nothing_is_spoken_during_the_journal_catch_up ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test bad_settings_are_reported_not_fatal ... ok
test quiet_during_combat ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test quiet_during_docking ... ok
test a_disabled_rule_never_fires ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test each_scan_type_gives_its_line ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 47 tests
test autostart::tests::command_line_quotes_the_path ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::bounce_gives_one_clip ... ok
test crash::tests::marker_round_trip ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::launcher_game_exit ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test logging::tests::rolls_over_and_prunes ... ok
test control::tests::round_trip_and_token ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test speech::tests::a_panicking_renderer_does_not_leave_the_speaker_busy ... ok
test speech::tests::busy_while_a_line_renders_or_plays_without_a_device ... ok
test tester::tests::lease_expires ... ok
test speech::tests::cache_names_are_stable ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::copy_skips_data_dir ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::version_order ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::plan_reflects_game_state ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test update::tests::signs_and_verifies_manifest ... ok
test callouts::tests::waits_while_the_speaker_is_busy ... ok
test callouts::tests::wait
```

## Stderr

Bytes: 4430
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.28s
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.05s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-227d14bf08a5f3c1)
     Running tests/prospector.rs (target/debug/deps/prospector-454c068e0f0a25e7)
     Running tests/replay.rs (target/debug/deps/replay-385379d9602fbb34)
     Running tests/scan.rs (target/debug/deps/scan-a90b84bee48dec26)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-65c8650de0c7f40d)
     Running unittests src/main.rs (target/debug/deps/astromech_core-3f60d293d99a077f)
     Running tests/gear_down.rs (target/debug/deps/gear_down-e916522f0f2600ae)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-509c18f5c7bf3a54)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-942fbfe589df60b0)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-21a24efd2b8cda42)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-b98b6d50850e8a1e)
   Doc-tests astromech_announce
   Doc-tests astromech_core
   Doc-tests astromech_engineering
   Doc-tests astromech_galaxy
   Doc-tests astromech_gamestate
   Doc-tests astromech_input
```

