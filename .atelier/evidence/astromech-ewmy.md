---
created_at: "2026-10-16T23:55:33.796988440+00:00"
id: "astromech-ewmy"
evidence_type: "test"
commit: "7ac7cdcd18f8c796c25f53ab717f0ff1ef5a7b54"
result: "pass"
captured_at: "2026-10-16T23:54:44.935878582+00:00"
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
title: "After merging main (k3wp colonisation) into task/astromech-mljg, gates: fmt, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked, app lint/check/build/test"
updated_at: "2026-10-16T23:55:33.797291806+00:00"
---

## Summary

After merging main (k3wp colonisation) into task/astromech-mljg, gates: fmt, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked, app lint/check/build/test

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 50205
Truncated: yes

```text

running 16 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::claims_and_beacons ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::corrupt_record_is_an_error_not_silent_loss ... ok
test tests::save_writes_only_changes ... ok
test tests::shopping_tests::cargo_json_overrides_the_replayed_hold ... ok
test tests::survives_restart ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 71 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test control::tests::text_argument_is_optional ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test control::tests::round_trip_and_token ... ok
test gamewatch::tests::launcher_game_exit ... ok
test calibration::tests::steps_and_errors ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test calibration::tests::level_check_verdicts ... ok
test logging::tests::rolls_over_and_prunes ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test gamewatch::tests::lists_processes ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::the_eddn_switch_is_saved_and_keeps_its_comment ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::preview_slot_lifecycle ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_an
```

## Stderr

Bytes: 9973
Truncated: yes

```text
    Checking astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-journal)
    Checking astromech-store v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-store)
    Checking astromech-keybinds v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-keybinds)
    Checking astromech-status v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-status)
    Checking astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-data)
    Checking astromech-proto v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-proto)
   Compiling astromech-voice v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-voice)
    Checking astromech-intent v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-intent)
    Checking astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-eddn)
    Checking astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-materials)
    Checking astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-powerplay)
    Checking astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-engineering)
    Checking astromech-input v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-input)
    Checking astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-history)
    Checking astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-gamestate)
    Checking astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-galaxy)
    Checking astromech-odyssey v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-odyssey)
    Checking astromech-policy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-policy)
    Checking astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-station)
    Checking astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-ship)
    Checking astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-market)
    Checking astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-tools)
    Checking astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-core)
    Checking astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-colonisation)
    Checking astromech-overlay v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-overlay)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.40s
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-mljg/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-mljg/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-materials)`
```

