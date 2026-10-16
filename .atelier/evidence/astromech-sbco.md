---
created_at: "2026-10-16T22:05:30.749818107+00:00"
id: "astromech-sbco"
evidence_type: "test"
commit: "8dde3e6c94c3137b55bdd95489102c9f5ca60a1a"
result: "pass"
captured_at: "2026-10-16T22:03:47.441843487+00:00"
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
title: "Full pre-PR checks after merging main (Powerplay, PR #29) at 8dde3e6: fmt, clippy -D warnings, workspace tests, dependency rules, shell Cargo.lock --locked, app lint/check/build/vitest"
updated_at: "2026-10-16T22:05:30.750348256+00:00"
---

## Summary

Full pre-PR checks after merging main (Powerplay, PR #29) at 8dde3e6: fmt, clippy -D warnings, workspace tests, dependency rules, shell Cargo.lock --locked, app lint/check/build/vitest

## Command

```console
sh -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 32083
Truncated: yes

```text

running 60 tests
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test control::tests::text_argument_is_optional ... ok
test calibration::tests::level_check_verdicts ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::bounce_gives_one_clip ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::launcher_game_exit ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test calibration::tests::steps_and_errors ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test control::tests::round_trip_and_token ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test logging::tests::rolls_over_and_prunes ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test gamewatch::tests::lists_processes ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test speech::tests::preview_slot_lifecycle ... ok
test setup::tests::requests_become_edits ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test tester::tests::lease_expires ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::copy_skips_data_dir ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test update::tests::version_order ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test update::tests::signs_and_verifies_manifest ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test tester::tests::plan_reflects_game_state ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test history::tests::imports_at_startup_then_follows_new_lines ... ok
test history::tests::follows_appends_without_notifications ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok

test result: ok. 60 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test no_keyboard_binding_refuses_clearly ... ok
test state_rules_press_nothing ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test silent_running_asks_then_runs ... ok
test confirmation_is_logged_with_latency ... ok
t
```

## Stderr

Bytes: 7903
Truncated: yes

```text
    Checking astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-journal)
    Checking astromech-store v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-store)
    Checking astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-data)
    Checking astromech-keybinds v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-keybinds)
    Checking astromech-input v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-input)
    Checking astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-materials)
    Checking astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-powerplay)
    Checking astromech-status v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-status)
    Checking astromech-proto v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-proto)
   Compiling astromech-voice v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-voice)
    Checking astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-gamestate)
    Checking astromech-policy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-policy)
    Checking astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-history)
    Checking astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-galaxy)
    Checking astromech-intent v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-intent)
    Checking astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-engineering)
    Checking astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-market)
    Checking astromech-overlay v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-overlay)
    Checking astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-tools)
    Checking astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-ship)
    Checking astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.42s
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-zf3s/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-zf3s/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see
```

