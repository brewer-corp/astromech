---
created_at: "2026-10-16T20:54:46.776312258+00:00"
id: "astromech-0f4k"
evidence_type: "check"
commit: "53fbc221b9282183e33c953f4a0409931f30fb62"
result: "pass"
captured_at: "2026-10-16T20:54:28.876824105+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --format-version 1 >/dev/null'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-tw2y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tw2y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "fmt, clippy -D warnings, workspace tests, check-deps, shell Cargo.lock current (--locked metadata)"
updated_at: "2026-10-16T20:54:46.776874768+00:00"
---

## Summary

fmt, clippy -D warnings, workspace tests, check-deps, shell Cargo.lock current (--locked metadata)

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets --locked -- -D warnings && cargo test --workspace --locked && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --format-version 1 >/dev/null'
```

Exit status: 0

## Stdout

Bytes: 18273
Truncated: yes

```text

running 40 tests
test autostart::tests::command_line_quotes_the_path ... ok
test control::tests::text_argument_is_optional ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test crash::tests::marker_round_trip ... ok
test config::tests::finds_look_alike_config_names ... ok
test autostart::tests::state_round_trip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::bounce_gives_one_clip ... ok
test logging::tests::rolls_over_and_prunes ... ok
test control::tests::round_trip_and_token ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test tester::tests::lease_expires ... ok
test speech::tests::cache_names_are_stable ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test update::tests::version_order ... ok
test update::tests::copy_skips_data_dir ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test update::tests::baked_public_key_parses ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test update::tests::signs_and_verifies_manifest ... ok
test tester::tests::plan_reflects_game_state ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::lookups_say_what_they_would_look_up ... ok
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test lookups::tests::service_phrases_resolve_to_lookups ... ok

test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test silent_running_asks_then_runs ... ok
test state_rules_press_nothing ... ok
test did_you_mean_then_yes_executes ... ok
test did_you_mean_yes_learns_alias ... ok
test voice_path_with_fake_stt ... ok
test real_game_round_two ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.59s


running 6 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test kill_switch_during_the_live_run_wait_presses_nothing ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.77s


running 3 tests
test transport::tests::segments_and_cache_keys ... ok
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test spansh::tests::sea
```

## Stderr

Bytes: 4203
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-m30a/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-m30a/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-4a617b92e0db4179)
     Running unittests src/main.rs (target/debug/deps/astromech_core-e5d7565602ce009a)
     Running tests/gear_down.rs (target/debug/deps/gear_down-7ce2d8bce3e5d9d7)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-6f9fc094339538be)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-942fbfe589df60b0)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-4d6019b18242c6a6)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-551a46c45e29fe62)
     Running tests/companions.rs (target/debug/deps/companions-da38797e99658c4f)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-764f9010b8c5d158)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-807f84173a65359d)
   Doc-tests astromech_core
   Doc-tests astromech_data
   Doc-tests astromech_engineering
   Doc-tests astromech_galaxy
   Doc-tests astromech_gamestate
   Doc-tests astromech_input
   Doc-tests astromech_intent
   Doc-tests astromech_journal
   Doc-tests astromech_keybinds
   Doc-tests astromech_materials
   Doc-tests astromech_overlay
   Doc-tests astromech_policy
   Doc-tests astromech_proto
   Doc-tes
```

