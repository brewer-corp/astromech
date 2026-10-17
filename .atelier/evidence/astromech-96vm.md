---
created_at: "2026-10-17T00:41:36.719589211+00:00"
id: "astromech-96vm"
evidence_type: "test"
commit: "a5d118dc7cb0c92618eeba9d08828e2b463ff494"
result: "pass"
captured_at: "2026-10-17T00:40:58.693174337+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-d1lo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-d1lo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full gates after merging main (a5d118d, trade route finder): fmt, clippy -D warnings, cargo test --workspace (601 passed), check-deps (26 crates), app/src-tauri cargo metadata --locked --offline, pnpm install --frozen-lockfile, lint, check, build, test (33 files, 147 tests)."
updated_at: "2026-10-17T00:41:36.720138256+00:00"
---

## Summary

Full gates after merging main (a5d118d, trade route finder): fmt, clippy -D warnings, cargo test --workspace (601 passed), check-deps (26 crates), app/src-tauri cargo metadata --locked --offline, pnpm install --frozen-lockfile, lint, check, build, test (33 files, 147 tests).

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 53956
Truncated: yes

```text

running 3 tests
test prospector::tests::thresholds_default_override_and_off ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test a_rock_below_threshold_gets_nothing ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_motherlode_always_gets_a_line ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test quiet_during_docking ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test a_disabled_rule_never_fires ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test quiet_during_combat ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test each_scan_type_gives_its_line ... ok
test a_shown_only_scan_does_not_swallow_the_urgent_stolen_goods_line ... ok
test stolen_goods_speak_through_docking_quiet_and_scans_repeat_once_per_cooldown ... ok
test the_stolen_goods_suffix_only_with_stolen_cargo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::claims_and_beacons ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::shopping_tests::cargo_json_overrides_the_replayed_hold ... ok
test tests::save_writes_only_changes ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::survives_restart ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 82 tests
test autostart::tests::command_line_quotes_the_path ... ok
test callouts::tests::keeps_the_last_toasts ... ok
test autostart::tests::state_round_trip ... ok
test calibration::tests::level_check_verdicts ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::bounce_gives_one_clip ... ok
test calibration::tests::steps_and_errors ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test
```

## Stderr

Bytes: 6540
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.92s
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-d1lo/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-d1lo/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/astromech_announce-0bd349a8d045c7fa)
     Running tests/prospector.rs (target/debug/deps/prospector-2c265afbf4ea3e1b)
     Running tests/replay.rs (target/debug/deps/replay-d65a1bd159f2d82c)
     Running tests/scan.rs (target/debug/deps/scan-573568d140cb84ea)
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-9ee2fa666954b1d4)
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3e75a7de3558b79f)
     Running unittests src/main.rs (target/debug/deps/astromech_core-51abc3700ef78a43)
     Running tests/copy.rs (target/debug/deps/copy-b75465027b55a8d6)
     Running tests/gear_down.rs (target/debug/deps/gear_down-f36560f5f22e0850)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-0cbcf7dab74137b3)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-0ee56d6e3b6197a5)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-32f11f6e11432948)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-6ed659280f0eb760)
     Running tests/recorded.rs (target/debug/deps/recorded-077bd2d8d758312c)
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-943ca53e72b74208)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ccdb1e86be63f36e)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-5811f7cd0f3a0534)
     Running tests/powerplay.rs (target/debug/deps/powerplay-279087c75a955c55)
     Running tests/sites.rs (target/debug/deps/sites-61e89b44860c1b5a)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_market-88765869bbf1ce07)
     Running tests/anywhere.rs (target/debug/deps/anywhere-8510892c1f499848)
     Runnin
```

