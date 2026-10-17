---
created_at: "2026-10-17T20:08:59.769777552+00:00"
id: "astromech-tyq0"
evidence_type: "test"
commit: "442b48233c6fb68ddb28959be8ea3965dd5c1f72"
result: "pass"
captured_at: "2026-10-17T20:08:52.985245510+00:00"
command: "cargo test -p astromech-core --lib app::tests"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-c9em"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-c9em"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib app::tests"
updated_at: "2026-10-17T20:08:59.770121764+00:00"
---

## Summary

cargo test -p astromech-core --lib app::tests

## Command

```console
cargo test -p astromech-core --lib app::tests
```

Exit status: 0

## Stdout

Bytes: 1235
Truncated: no

```text

running 14 tests
test app::tests::route_commands_over_the_control_api ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test app::tests::the_status_line_says_when_push_to_talk_is_only_testing ... ok
test app::tests::plotted_routes_load_into_the_follower ... ok
test app::tests::the_sample_config_written_by_open_config_changes_no_setting ... ok
test app::tests::a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout ... ok
test app::tests::a_journal_folder_set_after_startup_is_imported ... ok
test app::tests::set_eddn_flips_the_shared_switch_and_switching_off_empties_the_queue_at_once ... ok
test app::tests::callout_settings_round_trip_through_the_config_file ... ok
test app::tests::fixing_a_broken_config_clears_the_error_without_a_restart ... ok
test app::tests::edsm_settings_keep_the_key_in_the_keyring_and_upload_only_with_both ... ok
test app::tests::fixing_ignored_values_updates_then_clears_the_note_without_a_restart ... ok
test app::tests::setup_and_calibration_reply_shapes_match_the_window_fake ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 124 filtered out; finished in 6.53s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-c15521d722cb5c9a)
```

