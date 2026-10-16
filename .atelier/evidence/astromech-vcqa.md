---
created_at: "2026-10-16T20:55:43.326036450+00:00"
id: "astromech-vcqa"
evidence_type: "test"
commit: "0c11ea1b95630ff8bab98ac8204c4b5155ad226d"
result: "pass"
captured_at: "2026-10-16T20:55:15.952156508+00:00"
command: "bash -c 'set -e; export CARGO_BUILD_JOBS=3; cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace; python3 scripts/check-deps.py; cargo metadata --locked --format-version 1 >/dev/null; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo shell-lock-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-7ni2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-7ni2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main (0c11ea1): fmt, clippy -D warnings, cargo test --workspace, check-deps, root and shell Cargo.lock --locked, app install/lint/check/build/vitest. Not run: cargo deny (not installed), shell build/clippy (Windows CI only)."
updated_at: "2026-10-16T20:55:43.326512519+00:00"
---

## Summary

After merging main (0c11ea1): fmt, clippy -D warnings, cargo test --workspace, check-deps, root and shell Cargo.lock --locked, app install/lint/check/build/vitest. Not run: cargo deny (not installed), shell build/clippy (Windows CI only).

## Command

```console
bash -c 'set -e; export CARGO_BUILD_JOBS=3; cargo fmt --all -- --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace; python3 scripts/check-deps.py; cargo metadata --locked --format-version 1 >/dev/null; (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo shell-lock-ok); cd app; pnpm install --frozen-lockfile; pnpm lint; pnpm check; pnpm build; pnpm test'
```

Exit status: 0

## Stdout

Bytes: 27663
Truncated: yes

```text

running 57 tests
test autostart::tests::command_line_quotes_the_path ... ok
test control::tests::text_argument_is_optional ... ok
test autostart::tests::state_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test calibration::tests::level_check_verdicts ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::finds_look_alike_config_names ... ok
test crash::tests::marker_round_trip ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test control::tests::round_trip_and_token ... ok
test controls::tests::bounce_gives_one_clip ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::voice_settings_persist ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test speech::tests::cache_names_are_stable ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test speech::tests::preview_slot_lifecycle ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test tester::tests::lease_expires ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test calibration::tests::steps_and_errors ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::onboarding_flag ... ok
test update::tests::version_order ... ok
test update::tests::copy_skips_data_dir ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::plan_reflects_game_state ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test update::tests::signs_and_verifies_manifest ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok

test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test silent_running_asks_then_runs ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test state_rules_press_nothing ... ok
test did_you_mean_then_yes_executes ... ok
test real_game_round_two ... ok
test did_you_mean_yes_learns_alias ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ...
```

## Stderr

Bytes: 4749
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-7ni2/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-7ni2/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.12s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8f76410bebbb728c)
     Running unittests src/main.rs (target/debug/deps/astromech_core-fd142fb988bd6e86)
     Running tests/gear_down.rs (target/debug/deps/gear_down-c39a80d2c89adcf9)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-fd47c8178dea78f3)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-942fbfe589df60b0)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-b591a561b9f40aae)
     Running tests/powerplay.rs (target/debug/deps/powerplay-554269ec4e6822df)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-1f7fe0e1a53e3e05)
     Running tests/companions.rs (target/debug/deps/companions-16c3a42d06c4eb7f)
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
     Running tests/standing.rs (target/debug/deps/standing-c16e9ae08297b37e)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-f4c6dcf9c8519f66)
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-84e0e81be2efc563)
     Running tests/coriolis.rs (target/debug/deps/coriolis-670c639f19b2ffcc)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-6dfddf9596e437bb)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-5391961d6a6a6ea6)
     Runnin
```

