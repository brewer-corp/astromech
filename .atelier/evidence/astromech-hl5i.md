---
created_at: "2026-10-16T19:29:04.889985764+00:00"
id: "astromech-hl5i"
evidence_type: "test"
commit: "c1038d4e95054f4ae0f78919ddff5dcebf3071f5"
result: "pass"
captured_at: "2026-10-16T19:28:59.025484647+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-w20r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-w20r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "journal/1: a fixture of each of Docked, FSDJump, Scan, Location, SAASignalsFound, CarrierJump validates against live journal-v1.0.json (EDDN 4ad669bb) with the README elisions asserted absent (tests/journal_schema.rs); mutation-checked (removing Latitude/Longitude elision or Docked StarPos augment fails a test)"
updated_at: "2026-10-16T19:29:04.890217276+00:00"
---

## Summary

journal/1: a fixture of each of Docked, FSDJump, Scan, Location, SAASignalsFound, CarrierJump validates against live journal-v1.0.json (EDDN 4ad669bb) with the README elisions asserted absent (tests/journal_schema.rs); mutation-checked (removing Latitude/Longitude elision or Docked StarPos augment fails a test)

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 4550
Truncated: yes

```text

running 37 tests
test context::tests::partial_location_clears_the_fix ... ok
test context::tests::header_and_flags_come_from_fileheader_and_loadgame ... ok
test journal::tests::only_the_six_events_are_sent ... ok
test context::tests::new_session_fileheader_resets_everything ... ok
test context::tests::loadgame_supplies_version_without_a_fileheader ... ok
test context::tests::beta_versions_are_detected ... ok
test message::tests::augmentation_adds_fields_after_a_matching_cross_check ... ok
test message::tests::augmentation_mismatch_drops_the_message ... ok
test context::tests::continued_file_keeps_the_session ... ok
test message::tests::augmentation_without_anything_to_check_drops_the_message ... ok
test message::tests::beta_game_versions_are_not_built ... ok
test message::tests::augmentation_is_skipped_when_fields_are_present_or_not_asked_for ... ok
test message::tests::commander_data_is_stripped ... ok
test message::tests::every_system_name_key_is_cross_checked ... ok
test message::tests::flags_are_never_null ... ok
test journal::tests::events_missing_a_field_that_cannot_be_added_are_not_sent ... ok
test message::tests::per_event_elisions_apply_only_to_their_event ... ok
test message::tests::header_comes_from_fileheader_and_is_sent_even_when_empty ... ok
test message::tests::localised_keys_are_stripped_at_every_depth ... ok
test message::tests::unknown_game_version_is_not_built ... ok
test message::tests::test_mode_uses_the_test_schema ... ok
test tests::beta_messages_are_never_queued_or_sent ... ok
test outbox::tests::sent_messages_leave_the_outbox_and_are_logged ... ok
test outbox::tests::http_426_is_not_retried ... ok
test outbox::tests::a_failure_mid_send_does_not_leave_the_message_due ... ok
test outbox::tests::http_400_is_not_retried ... ok
test outbox::tests::one_bad_message_does_not_block_the_others ... ok
test outbox::tests::outbox_survives_a_restart ... ok
test outbox::tests::backoff_doubles_from_a_minute_and_gives_up ... ok
test transport::tests::connection_failure_is_an_error_without_status ... ok
test outbox::tests::other_failures_retry_no_sooner_than_one_minute ... ok
test transport::tests::long_multibyte_body_is_clipped_on_a_char_boundary ... ok
test transport::tests::posts_gzip_with_the_required_headers_and_returns_error_statuses ... ok
test tests::submitted_message_is_posted_gzipped_to_the_test_schema ... ok
test outbox::tests::send_log_is_capped ... ok
test message::tests::encode_gzips_and_refuses_bodies_over_one_mib ... ok
test tests::message_over_one_mib_gzipped_is_dropped_and_logged ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.36s


running 17 tests
test a_run_with_only_other_systems_or_mission_targets_sends_nothing ... ok
test fssdiscoveryscan_naming_another_system_is_not_built ... ok
test navbeaconscan_adds_starsystem_and_starpos ... ok
test navroute_comes_from_the_announced_file ... ok
test separate_runs_are_separate_messages ... ok
test fssallbodiesfound_adds_starpos ... ok
test dockinggranted_and_dockingdenied_validate ... ok
test flush_sends_a_pending_run_and_drops_it_without_a_location ... ok
test a_run_of_signals_is_one_message_ended_by_the_next_other_event ... ok
test fssdiscoveryscan_drops_progress_and_adds_starpos ... ok
test scanbarycentre_adds_starpos ... ok
test fssbodysignals_adds_starsystem_and_starpos ... ok
test fcmaterials_journal_comes_from_the_announced_file ... ok
test approachsettlement_keeps_latitude_and_longitude_and_adds_the_system ... ok
test signals_before_an_fsdjump_are_checked_against_the_new_system ... ok
test the_batch_comes_before_the_ending_events_own_message ... ok
test codexentry_gets_bodyid_only_when_status_and_journal_agree ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 8 tests
test scan_from_another_system_is_dropped_not_augmented ... ok
test fsdjump_validates_with_jump_and_faction_personal_keys_gone ... ok
test location_validates_with_latitude_longitude_and_wanted_gone ... ok
test saasignalsfound_validates_with_starsystem_and_starpos_added .
```

## Stderr

Bytes: 490
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz-journal/crates/astromech-eddn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.30s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-56ff1c1f80a1c470)
     Running tests/event_schemas.rs (target/debug/deps/event_schemas-21bd050fce22acd6)
     Running tests/journal_schema.rs (target/debug/deps/journal_schema-3545d2285e41eff6)
   Doc-tests astromech_eddn
```

