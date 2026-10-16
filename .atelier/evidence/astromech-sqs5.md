---
created_at: "2026-10-16T19:50:47.489600268+00:00"
id: "astromech-sqs5"
evidence_type: "test"
commit: "d5215195f8d11868b394cc294138b0c3dbaab4c0"
result: "pass"
captured_at: "2026-10-16T19:50:23.015551899+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-qz9u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qz9u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main (b570ef8) and adding vitest tests (d521519): fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, and in app/ pnpm install --frozen-lockfile, lint, check, build, test (filterstate.test.ts; SearchKit.test.ts load-failure fallback, which fails on the pre-fix wiring) all pass. Shell not compiled on Linux."
updated_at: "2026-10-16T19:50:47.490380646+00:00"
---

## Summary

After merging main (b570ef8) and adding vitest tests (d521519): fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, and in app/ pnpm install --frozen-lockfile, lint, check, build, test (filterstate.test.ts; SearchKit.test.ts load-failure fallback, which fails on the pre-fix wiring) all pass. Shell not compiled on Linux.

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 23453
Truncated: yes

```text

running 37 tests
test autostart::tests::state_round_trip ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::bounce_gives_one_clip ... ok
test config::tests::finds_look_alike_config_names ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test crash::tests::marker_round_trip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test logging::tests::rolls_over_and_prunes ... ok
test gamewatch::tests::launcher_game_exit ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test control::tests::round_trip_and_token ... ok
test tester::tests::lease_expires ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test speech::tests::cache_names_are_stable ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test update::tests::copy_skips_data_dir ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::baked_public_key_parses ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test update::tests::version_order ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test update::tests::signs_and_verifies_manifest ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::plan_reflects_game_state ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test silent_running_asks_then_runs ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test state_rules_press_nothing ... ok
test did_you_mean_yes_learns_alias ... ok
test real_game_round_two ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.55s


running 6 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test kill_switch_during_the_live_run_wait_presses_nothing ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.72s


running 3 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok
test transport::tests::segments_and_cache_keys ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 12 tests
test spansh_route_job_round_trip ... ok
test http_erro
```

## Stderr

Bytes: 4203
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-qz9u/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-qz9u/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-b6c00dba852f455e)
     Running unittests src/main.rs (target/debug/deps/astromech_core-ce64816f2e4fd3ec)
     Running tests/gear_down.rs (target/debug/deps/gear_down-535ee79e56361fb4)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-144d638b19df2445)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-47de08a4b04b1860)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-27fbc016e30e0fd2)
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

