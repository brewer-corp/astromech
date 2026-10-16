---
created_at: "2026-10-16T19:30:53.009460533+00:00"
id: "astromech-s2kj"
evidence_type: "test"
commit: "11d78d76f3aa3eecdead7c7525351fa539eaf35d"
result: "pass"
captured_at: "2026-10-16T19:30:51.995306322+00:00"
command: "cargo test -p astromech-core --lib"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mmqs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mmqs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Re-review fixes N1-N5: wizard-written PTT then restart stays not onboarded (fails without the migrated key), upgrader still skips onboarding, fresh install and sample config need onboarding, no temp file left"
updated_at: "2026-10-16T19:30:53.009674533+00:00"
---

## Summary

Re-review fixes N1-N5: wizard-written PTT then restart stays not onboarded (fails without the migrated key), upgrader still skips onboarding, fresh install and sample config need onboarding, no temp file left

## Command

```console
cargo test -p astromech-core --lib
```

Exit status: 0

## Stdout

Bytes: 2558
Truncated: no

```text

running 43 tests
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::bounce_gives_one_clip ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::game_without_launcher ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test calibration::tests::steps_and_errors ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::launcher_game_exit ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test logging::tests::rolls_over_and_prunes ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test calibration::tests::level_check_verdicts ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test control::tests::round_trip_and_token ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::requests_become_edits ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::baked_public_key_parses ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test update::tests::version_order ... ok
test update::tests::copy_skips_data_dir ... ok
test setup::tests::onboarding_flag ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test update::tests::signs_and_verifies_manifest ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok

test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
```

## Stderr

Bytes: 278
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8c448b21aa4d9de9)
```

