---
created_at: "2026-10-16T19:17:18.695395774+00:00"
id: "astromech-neuk"
evidence_type: "test"
commit: "4a43d0804ae0e8b29ee9beba1cf1d4ef0ba0909c"
result: "pass"
captured_at: "2026-10-16T19:17:18.402337983+00:00"
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
title: "Core side of onboarding (setup:: and calibration:: tests in the lib): find_elite with no install / missing / found folders, config edits keep comments and refuse to overwrite a broken file, request-to-edit parsing, onboarding flag, mic level check and calibration with no mic"
updated_at: "2026-10-16T19:17:18.695848999+00:00"
---

## Summary

Core side of onboarding (setup:: and calibration:: tests in the lib): find_elite with no install / missing / found folders, config edits keep comments and refuse to overwrite a broken file, request-to-edit parsing, onboarding flag, mic level check and calibration with no mic

## Command

```console
cargo test -p astromech-core --lib
```

Exit status: 0

## Stdout

Bytes: 2205
Truncated: no

```text

running 38 tests
test autostart::tests::command_line_quotes_the_path ... ok
test calibration::tests::device_name_matches_what_the_session_opens ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::debug_input_logs_every_button ... ok
test controls::tests::bounce_gives_one_clip ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test gamewatch::tests::game_without_launcher ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test gamewatch::tests::launcher_game_exit ... ok
test calibration::tests::steps_and_errors ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test crash::tests::marker_round_trip ... ok
test logging::tests::rolls_over_and_prunes ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::requests_become_edits ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test control::tests::round_trip_and_token ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::baked_public_key_parses ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test calibration::tests::level_check_verdicts ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::version_order ... ok
test gamewatch::tests::lists_processes ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test update::tests::signs_and_verifies_manifest ... ok
test calibration::tests::window_and_cli_store_the_same_gate ... ok

test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8c448b21aa4d9de9)
```

