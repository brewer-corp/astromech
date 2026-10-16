---
created_at: "2026-10-16T20:39:55.065801802+00:00"
id: "astromech-ndnm"
evidence_type: "test"
commit: "391bf16e5c12553c6ad88914935f085d755a6c76"
result: "fail"
captured_at: "2026-10-16T20:39:46.571090882+00:00"
command: "sh -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "101"
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
title: "Full pre-PR checks: fmt, clippy -D warnings, workspace tests, dependency rules, shell Cargo.lock --locked, app lint/check/build/vitest"
updated_at: "2026-10-16T20:39:55.066027814+00:00"
---

## Summary

Full pre-PR checks: fmt, clippy -D warnings, workspace tests, dependency rules, shell Cargo.lock --locked, app lint/check/build/vitest

## Command

```console
sh -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 101

## Stdout

Bytes: 3472
Truncated: no

```text

running 37 tests
test autostart::tests::command_line_quotes_the_path ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test control::tests::text_argument_is_optional ... ok
test autostart::tests::state_round_trip ... ok
test control::tests::round_trip_and_token ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test controls::tests::bounce_gives_one_clip ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::cache_names_are_stable ... ok
test tester::tests::lease_expires ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::finds_look_alike_config_names ... ok
test gamewatch::tests::finds_newest_journal ... ok
test logging::tests::rolls_over_and_prunes ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::version_order ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test update::tests::baked_public_key_parses ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::plan_reflects_game_state ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test state_rules_press_nothing ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test silent_running_asks_then_runs ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test did_you_mean_yes_learns_alias ... ok
test real_game_round_two ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... FAILED
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

failures:

---- confirmation_is_logged_with_latency stdout ----

thread 'confirmation_is_logged_with_latency' (568450) panicked at crates/astromech-core/tests/gear_down.rs:169:5:
549
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    confirmation_is_logged_with_latency

test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.61s
```

## Stderr

Bytes: 1868
Truncated: no

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.10s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-0a68776badf13898)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8eb57155e1e03a88)
     Running tests/gear_down.rs (target/debug/deps/gear_down-13648337056538c0)
error: test failed, to rerun pass `-p astromech-core --test gear_down`
```

