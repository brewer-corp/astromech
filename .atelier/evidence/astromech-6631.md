---
created_at: "2026-10-16T22:23:45.960938521+00:00"
id: "astromech-6631"
evidence_type: "test"
commit: "b1715cf5d0606c060a839057dcac49dfd1c59976"
result: "pass"
captured_at: "2026-10-16T22:23:17.367179744+00:00"
command: "bash /home/supadava/.cache/zig3-fix/gates.sh"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-zig3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zig3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "All gates at merged head: fmt, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, pnpm lint/check/build/test, atelier check"
updated_at: "2026-10-16T22:23:45.961470944+00:00"
---

## Summary

All gates at merged head: fmt, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, pnpm lint/check/build/test, atelier check

## Command

```console
bash /home/supadava/.cache/zig3-fix/gates.sh
```

Exit status: 0

## Stdout

Bytes: 39628
Truncated: yes

```text

running 65 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::state_round_trip ... ok
test control::tests::text_argument_is_optional ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test controls::tests::debug_input_logs_every_button ... ok
test crash::tests::marker_round_trip ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test gamewatch::tests::finds_newest_journal ... ok
test gamewatch::tests::game_without_launcher ... ok
test control::tests::round_trip_and_token ... ok
test gamewatch::tests::launcher_game_exit ... ok
test calibration::tests::steps_and_errors ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test logging::tests::rolls_over_and_prunes ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test calibration::tests::level_check_verdicts ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test gamewatch::tests::lists_processes ... ok
test setup::tests::onboarding_flag ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::preview_slot_lifecycle ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test tester::tests::lease_expires ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test setup::tests::the_eddn_switch_is_saved_and_keeps_its_comment ... ok
test update::tests::baked_public_key_parses ... ok
test update::tests::copy_skips_data_dir ... ok
test tester::tests::noise_and_nonsense_have_no_keys ... ok
test update::tests::version_order ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test update::tests::signs_and_verifies_manifest ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test tester::tests::unavailable_and_did_you_mean ... ok
test tester::tests::plan_reflects_game_state ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
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

Bytes: 5154
Truncated: yes

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-zig3/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-zig3/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-zig3/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-task-astromech-zig3/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-zig3/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-zig3/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-dfbb01eaac267683)
     Running unittests src/main.rs (target/debug/deps/astromech_core-fdfe9cbfa14ed794)
     Running tests/gear_down.rs (target/debug/deps/gear_down-cd21ffba65244650)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-f601c6141bbd35d4)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-bf95005bdf3019c6)
     Running tests/recorded.rs (target/debug/deps/recorded-80d516da36ee0b80)
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-9e9c786a955c74be)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-9853607d0eb0b74b)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-179b273388736436)
     Running tests/powerplay.rs (target/debug/deps/powerplay-fbc56d67f8f2ccf1)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-641f5787418d3923)
     Running tests/companions.rs (target/debug/deps/companions-725ffa961c4dfc13)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-12c154904ddbf506)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-96c9823824005daa)
     Running tests/listen.rs (target/debug/deps/listen-471f8114a4c1a654)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-d6676a1c5bf63860)
     Running tests/events.rs (target/debug/deps/events-d0979b58cfde7e67)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-cf12baa3e1fdee4f)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-fb07dc04605130f2)
     Running tests/live_journal.rs (target/debug/deps/live_journal-2693deaab3ba4288)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-9e393abaef84edea)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-b27679dce68d9b88)
     Running unittests src/lib.rs (target/debug/deps/astromech_powerplay-df1165627369c35f)
     Running tests/arrival.rs (target/debug/deps/arrival-cd3773b734e30f11)
     Running tests/standing.rs (target/debug/deps/standing-cb225ab6c59fe911)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-60375e975e5f5d2e)
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-e3b6725b64d1e96c)
     Running tests/coriolis.rs (target/debug/deps/coriolis-56dde5f6b962e87f)
     Running unittests src/lib.rs (target/debug/deps/astromech_station-6801229114e5dda3)
     Running tests/dock.rs
```

