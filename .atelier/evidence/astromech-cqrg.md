---
created_at: "2026-10-16T20:40:39.179449010+00:00"
id: "astromech-cqrg"
evidence_type: "test"
commit: "391bf16e5c12553c6ad88914935f085d755a6c76"
result: "pass"
captured_at: "2026-10-16T20:40:16.675896496+00:00"
command: "sh -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-xqdl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xqdl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full pre-PR checks (rerun after a timing flake in gear_down confirmation_is_logged_with_latency, 549 ms vs 550 ms floor under load avg 18; unrelated to this change): fmt, clippy -D warnings, workspace tests, dependency rules, shell Cargo.lock --locked, app lint/check/build/vitest"
updated_at: "2026-10-16T20:40:39.179812339+00:00"
---

## Summary

Full pre-PR checks (rerun after a timing flake in gear_down confirmation_is_logged_with_latency, 549 ms vs 550 ms floor under load avg 18; unrelated to this change): fmt, clippy -D warnings, workspace tests, dependency rules, shell Cargo.lock --locked, app lint/check/build/vitest

## Command

```console
sh -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 25413
Truncated: yes

```text

running 37 tests
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test config::tests::finds_look_alike_config_names ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::bounce_gives_one_clip ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test speech::tests::cache_names_are_stable ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test tester::tests::lease_expires ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test update::tests::baked_public_key_parses ... ok
test logging::tests::rolls_over_and_prunes ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test update::tests::version_order ... ok
test update::tests::copy_skips_data_dir ... ok
test control::tests::round_trip_and_token ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test gamewatch::tests::lists_processes ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test tester::tests::plan_reflects_game_state ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test state_rules_press_nothing ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test did_you_mean_then_yes_executes ... ok
test status_questions_are_spoken_not_pressed ... ok
test silent_running_asks_then_runs ... ok
test did_you_mean_yes_learns_alias ... ok
test voice_path_with_fake_stt ... ok
test real_game_round_two ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.53s


running 5 tests
test where_can_i_sell_uses_the_remembered_filters ... ok
test a_misheard_commodity_asks_first_then_answers ... ok
test where_can_i_buy_answers_the_cheapest_with_its_age ... ok
test no_location_or_no_network_are_said_plainly ... ok
test the_voice_tester_says_what_it_would_look_up_without_asking_ardent ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s


running 6 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test kill_switch_during_the_live_run_wait_presses_nothing ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 5 passed; 0 failed;
```

## Stderr

Bytes: 4481
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-zf3s/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-zf3s/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-0a68776badf13898)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8eb57155e1e03a88)
     Running tests/gear_down.rs (target/debug/deps/gear_down-13648337056538c0)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-a2ede73589f39130)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-fef6db83c9f53e85)
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
     Running unittests src/lib.rs (target/debug/deps/astromech_market-ce7ddcca47ca3edb)
     Running tests/recorded.rs (target/debug/deps/recorded-5b1decbe1eee8cb2)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-8416b38c54a0aceb)
     Running tests/live_journal.rs (target/debug/deps/live_journal-b2b1dd777f8a5d42)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-c73768c8bd01a11f)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-d7bc19546b5b1b63)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-f4c6dcf9c8519f66)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-6dfddf9596e437bb)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-1f56ada8bc916625)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-807f84173a65359d)
   Doc-tests astromech_core
   Doc-tests astromech_data
   Doc-tests astromech_engineering
   Doc-tests astromech_galaxy
   Doc-tests astromech_gamestate
   Doc-
```

