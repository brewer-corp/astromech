---
created_at: "2026-10-16T19:23:11.386809267+00:00"
id: "astromech-s7im"
evidence_type: "test"
commit: "bbb20ded0967efca1e3390b2b306c3eae75fe020"
result: "pass"
captured_at: "2026-10-16T19:23:09.347518567+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-lt2b"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-lt2b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "commodity/3: fixture Market.json builds a message that validates against the live commodity-v3.0.json (EDDN live 4ad669bb); README rules tested (NonMarketable skipped, name stripped, Producer/Rare/id/Category/StationType gone, no economies/prohibited, file/event match); mutations caught"
updated_at: "2026-10-16T19:23:11.387088008+00:00"
---

## Summary

commodity/3: fixture Market.json builds a message that validates against the live commodity-v3.0.json (EDDN live 4ad669bb); README rules tested (NonMarketable skipped, name stripped, Producer/Rare/id/Category/StationType gone, no economies/prohibited, file/event match); mutations caught

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 3591
Truncated: no

```text

running 49 tests
test companion::tests::other_events_are_not_station_events ... ok
test companion::tests::outfitting_skips_commander_sku_items_but_not_horizons_ones ... ok
test companion::tests::outfitting_with_nothing_left_is_not_sent ... ok
test companion::tests::outfitting_keeps_only_station_modules ... ok
test companion::tests::malformed_files_send_nothing ... ok
test companion::tests::commodity_follows_the_readme ... ok
test companion::tests::symbol_stripping ... ok
test companion::tests::file_from_another_visit_or_event_is_refused ... ok
test context::tests::beta_versions_are_detected ... ok
test context::tests::continued_file_keeps_the_session ... ok
test context::tests::header_and_flags_come_from_fileheader_and_loadgame ... ok
test context::tests::loadgame_supplies_version_without_a_fileheader ... ok
test context::tests::new_session_fileheader_resets_everything ... ok
test context::tests::partial_location_clears_the_fix ... ok
test message::tests::augmentation_is_skipped_when_fields_are_present_or_not_asked_for ... ok
test message::tests::augmentation_adds_fields_after_a_matching_cross_check ... ok
test message::tests::augmentation_mismatch_drops_the_message ... ok
test message::tests::augmentation_without_anything_to_check_drops_the_message ... ok
test message::tests::beta_game_versions_are_not_built ... ok
test message::tests::commander_data_is_stripped ... ok
test message::tests::flags_are_never_null ... ok
test companion::tests::shipyard_lists_ship_types_and_cobra_flag ... ok
test message::tests::header_comes_from_fileheader_and_is_sent_even_when_empty ... ok
test message::tests::localised_keys_are_stripped_at_every_depth ... ok
test message::tests::per_event_elisions_apply_only_to_their_event ... ok
test message::tests::unknown_game_version_is_not_built ... ok
test message::tests::test_mode_uses_the_test_schema ... ok
test outbox::tests::http_400_is_not_retried ... ok
test outbox::tests::http_426_is_not_retried ... ok
test outbox::tests::a_failure_mid_send_does_not_leave_the_message_due ... ok
test outbox::tests::one_bad_message_does_not_block_the_others ... ok
test outbox::tests::sent_messages_leave_the_outbox_and_are_logged ... ok
test tests::beta_messages_are_never_queued_or_sent ... ok
test outbox::tests::backoff_doubles_from_a_minute_and_gives_up ... ok
test companion::tests::shipyard_fixture_validates_against_shipyard_v2 ... ok
test outbox::tests::outbox_survives_a_restart ... ok
test companion::tests::market_fixture_validates_against_commodity_v3 ... ok
test transport::tests::connection_failure_is_an_error_without_status ... ok
test testutil::tests::schema_errors_reports_violations ... ok
test transport::tests::long_multibyte_body_is_clipped_on_a_char_boundary ... ok
test transport::tests::posts_gzip_with_the_required_headers_and_returns_error_statuses ... ok
test companion::tests::outfitting_fixture_validates_against_outfitting_v2 ... ok
test tests::submitted_message_is_posted_gzipped_to_the_test_schema ... ok
test companion::tests::carrier_docking_access_is_renamed ... ok
test outbox::tests::other_failures_retry_no_sooner_than_one_minute ... ok
test companion::tests::schema_rejects_what_the_readme_elides ... ok
test outbox::tests::send_log_is_capped ... ok
test message::tests::encode_gzips_and_refuses_bodies_over_one_mib ... ok
test tests::message_over_one_mib_gzipped_is_dropped_and_logged ... ok

test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.33s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 306
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-eddn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-56ff1c1f80a1c470)
   Doc-tests astromech_eddn
```

