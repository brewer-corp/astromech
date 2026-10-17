---
created_at: "2026-10-17T19:28:59.177840642+00:00"
id: "astromech-2gif"
evidence_type: "test"
commit: "6c1da6c33fdd85fc423918ce90e227a379953b2a"
result: "pass"
captured_at: "2026-10-17T19:28:50.743315043+00:00"
command: "cargo test -p astromech-core --lib callout"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-s0gz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-s0gz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib callout"
updated_at: "2026-10-17T19:28:59.178279521+00:00"
---

## Summary

cargo test -p astromech-core --lib callout

## Command

```console
cargo test -p astromech-core --lib callout
```

Exit status: 0

## Stdout

Bytes: 1082
Truncated: no

```text

running 12 tests
test callouts::tests::keeps_the_last_toasts ... ok
test callouts::tests::a_voice_turn_ends_with_its_jobs_or_after_the_limit ... ok
test setup::tests::callout_edits_write_the_table_and_remove_it_when_empty ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test callouts::tests::the_shopping_list_is_kept_per_ship_and_merged ... ok
test callouts::tests::prospector_thresholds_are_stored_and_changed_from_settings ... ok
test app::tests::callout_settings_round_trip_through_the_config_file ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok
test callouts::tests::waits_while_the_speaker_is_busy ... ok
test callouts::tests::waits_while_push_to_talk_is_held_and_until_the_reply_is_queued ... ok
test callouts::tests::notifications_run_in_the_background_at_most_one_per_gap ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 112 filtered out; finished in 1.51s
```

## Stderr

Bytes: 478
Truncated: no

```text
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-config/crates/astromech-announce)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-config/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-config/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.59s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
```

