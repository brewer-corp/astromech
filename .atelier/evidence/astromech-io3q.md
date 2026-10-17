---
created_at: "2026-10-16T23:36:15.409817582+00:00"
id: "astromech-io3q"
evidence_type: "test"
commit: "ba77ef85bf23f20da5764d5aa18a93113fe17139"
result: "pass"
captured_at: "2026-10-16T23:35:37.607886635+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mljg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mljg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Gates: cargo fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked, app lint/check/build/test"
updated_at: "2026-10-16T23:36:15.410319359+00:00"
---

## Summary

Gates: cargo fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked, app lint/check/build/test

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 43476
Truncated: yes

```text

running 65 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::debug_input_logs_every_button ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test crash::tests::marker_round_trip ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test calibration::tests::steps_and_errors ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test calibration::tests::level_check_verdicts ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test control::tests::round_trip_and_token ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test gamewatch::tests::lists_processes ... ok
test setup::tests::onboarding_flag ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::requests_become_edits ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::preview_slot_lifecycle ... ok
test setup::tests::the_eddn_switch_is_saved_and_keeps_its_comment ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test tester::tests::lease_expires ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::copy_skips_data_dir ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test update::tests::version_order ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test update::tests::signs_and_verifies_manifest ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::plan_reflects_game_state ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok

test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 meas
```

## Stderr

Bytes: 5623
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.67s
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-mljg/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-mljg/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-e24bf8cd150e1bac)
     Running unittests src/main.rs (target/debug/deps/astromech_core-2f4feec80b9544ff)
     Running tests/gear_down.rs (target/debug/deps/gear_down-a0803f95d4490193)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-a4e1bcca36e6b261)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-839004e0446ec577)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-6ed659280f0eb760)
     Running tests/recorded.rs (target/debug/deps/recorded-077bd2d8d758312c)
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-943ca53e72b74208)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ccdb1e86be63f36e)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-5811f7cd0f3a0534)
     Running tests/powerplay.rs (target/debug/deps/powerplay-7be4710b206f5244)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-ed6a7f4851c962d5)
     Running tests/companions.rs (target/debug/deps/companions-2addafdeb7d31f7a)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-24c9bd3f58aacc1d)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-71818ae170a2b90a)
     Running tests/listen.rs (target/debug/deps/listen-aefcf94f21ae37fb)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-49143e7857eb60de)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-45c2bd546db5006a)
     Running tests/events.rs (target/debug/deps/events-71fc1ec250dbbd8f)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-7a9ff47410104a2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_market-fb807e578ef26c19)
     Running tests/recorded.rs (target/debug/deps/recorded-1d149e13c60099e7)
     Running tests/route.rs (target/debug/deps/route-dbcd13b552e646de)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-0267425cef0ef5e8)
     Running tests/live_journal.rs (target/debug/deps/live_journal-a4363f75cba5e1c5)
     Running unittests src/lib.rs (target/debug/deps/astromech_odyssey-6ccf387f1aa849af)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-890262c0ad3b86d0)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-44c58fa828b7f665)
     Running unittests src/lib.rs (target/debug/deps/astromech_powerplay-4e50b23b05138239)
     Running tests/arrival.rs (target/debug/deps/arrival-250f0fffcfc1299e)
     Running tests/standing.rs (tar
```

