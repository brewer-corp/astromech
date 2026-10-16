---
created_at: "2026-10-16T20:01:55.023285313+00:00"
id: "astromech-ckvr"
evidence_type: "test"
commit: "06e57df93c762e3446fcf1301685994c036625c7"
result: "pass"
captured_at: "2026-10-16T20:01:51.912447120+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-xvjs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xvjs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-eddn tests at 06e57df: feed (replay from Fileheader sends nothing, new journal live from its Fileheader, companion files only for live events, Unusable and missing-key skips logged, outfitting gated, replayed signal run dropped, queuing never waits for a blocked post), outbox (pass stops at first connection error/408/500/503 and keeps untried messages' tries; retries=false sends only untried; dropped rows capped apart), worker (wall-clock jump doesn't cause an early retry; restart holds retries a minute)"
updated_at: "2026-10-16T20:01:55.023693254+00:00"
---

## Summary

astromech-eddn tests at 06e57df: feed (replay from Fileheader sends nothing, new journal live from its Fileheader, companion files only for live events, Unusable and missing-key skips logged, outfitting gated, replayed signal run dropped, queuing never waits for a blocked post), outbox (pass stops at first connection error/408/500/503 and keeps untried messages' tries; retries=false sends only untried; dropped rows capped apart), worker (wall-clock jump doesn't cause an early retry; restart holds retries a minute)

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 7082
Truncated: yes

```text

running 93 tests
test companion::tests::other_events_are_not_station_events ... ok
test companion::tests::outfitting_keeps_only_station_modules ... ok
test companion::tests::commodity_follows_the_readme ... ok
test companion::tests::outfitting_skips_commander_sku_items_but_not_horizons_ones ... ok
test companion::tests::file_from_another_visit_or_event_is_refused ... ok
test companion::tests::symbol_stripping ... ok
test companion::tests::outfitting_with_nothing_left_is_not_sent ... ok
test context::tests::beta_versions_are_detected ... ok
test context::tests::header_and_flags_come_from_fileheader_and_loadgame ... ok
test context::tests::loadgame_supplies_version_without_a_fileheader ... ok
test context::tests::continued_file_keeps_the_session ... ok
test context::tests::new_session_fileheader_resets_everything ... ok
test context::tests::partial_location_clears_the_fix ... ok
test companion::tests::shipyard_lists_ship_types_and_cobra_flag ... ok
test companion::tests::malformed_files_send_nothing ... ok
test events::schema_tests::a_run_with_only_other_systems_or_mission_targets_sends_nothing ... ok
test companion::tests::shipyard_fixture_validates_against_shipyard_v2 ... ok
test events::schema_tests::fssallbodiesfound_adds_starpos ... ok
test events::schema_tests::a_run_of_signals_is_one_message_ended_by_the_next_other_event ... ok
test companion::tests::outfitting_fixture_validates_against_outfitting_v2 ... ok
test events::schema_tests::dockinggranted_and_dockingdenied_validate ... ok
test events::schema_tests::flush_sends_a_pending_run_and_drops_it_without_a_location ... ok
test companion::tests::market_fixture_validates_against_commodity_v3 ... ok
test companion::tests::carrier_docking_access_is_renamed ... ok
test events::schema_tests::approachsettlement_keeps_latitude_and_longitude_and_adds_the_system ... ok
test events::schema_tests::fssdiscoveryscan_naming_another_system_is_not_built ... ok
test events::schema_tests::separate_runs_are_separate_messages ... ok
test events::schema_tests::the_batch_comes_before_the_ending_events_own_message ... ok
test events::schema_tests::fcmaterials_journal_comes_from_the_announced_file ... ok
test events::schema_tests::fssbodysignals_adds_starsystem_and_starpos ... ok
test events::schema_tests::fssdiscoveryscan_drops_progress_and_adds_starpos ... ok
test events::schema_tests::scanbarycentre_adds_starpos ... ok
test events::schema_tests::navroute_comes_from_the_announced_file ... ok
test events::schema_tests::signals_before_an_fsdjump_are_checked_against_the_new_system ... ok
test feed::tests::an_unusable_companion_file_is_logged ... ok
test feed::tests::a_live_journal_without_its_fileheader_sends_nothing ... ok
test feed::tests::outfitting_is_not_sent_yet ... ok
test companion::tests::schema_rejects_what_the_readme_elides ... ok
test events::schema_tests::navbeaconscan_adds_starsystem_and_starpos ... ok
test feed::tests::a_new_journal_is_read_from_its_fileheader ... ok
test feed::tests::a_signal_run_from_the_replay_is_not_sent_and_a_live_one_is_flushed ... ok
test feed::tests::a_replayed_journal_sets_the_context_but_sends_nothing ... ok
test feed::tests::skipped_events_are_logged ... ok
test journal::schema_tests::scan_from_another_system_is_dropped_not_augmented ... ok
test feed::tests::companion_files_are_sent_only_for_live_events ... ok
test feed::tests::navroute_waits_for_the_file_its_event_announced ... ok
test journal::tests::events_missing_a_field_that_cannot_be_added_are_skipped_with_a_reason ... ok
test journal::tests::only_the_six_events_are_sent ... ok
test message::tests::augmentation_adds_fields_after_a_matching_cross_check ... ok
test message::tests::augmentation_is_skipped_when_fields_are_present_or_not_asked_for ... ok
test message::tests::augmentation_mismatch_drops_the_message ... ok
test message::tests::augmentation_without_anything_to_check_drops_the_message ... ok
test message::tests::commander_data_is_stripped ... ok
test journal::schema_tests::fsdjump_validates_with_jump_and_faction_personal_keys_gone ... ok
test journal::schema_tests::docked_validates_wi
```

## Stderr

Bytes: 311
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz-wire/crates/astromech-eddn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.63s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-56ff1c1f80a1c470)
   Doc-tests astromech_eddn
```

