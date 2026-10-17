---
created_at: "2026-10-17T22:37:52.871242758+00:00"
id: "astromech-09u0"
evidence_type: "test"
commit: "4343918a94ca2177d022b8b234e2cafa210ac137"
result: "pass"
captured_at: "2026-10-17T22:37:50.391428004+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ix2p"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ix2p"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-eddn"
updated_at: "2026-10-17T22:37:52.871583834+00:00"
---

## Summary

cargo test -p astromech-eddn

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 11054
Truncated: yes

```text

running 143 tests
test companion::tests::file_from_another_visit_or_event_is_refused ... ok
test companion::tests::commodity_follows_the_readme ... ok
test companion::tests::other_events_are_not_station_events ... ok
test companion::tests::malformed_files_send_nothing ... ok
test companion::tests::outfitting_keeps_only_station_modules ... ok
test companion::tests::shipyard_lists_ship_types_and_cobra_flag ... ok
test context::tests::beta_versions_are_detected ... ok
test context::tests::continued_file_keeps_the_session ... ok
test companion::tests::outfitting_skips_commander_sku_items_but_not_horizons_ones ... ok
test companion::tests::outfitting_with_nothing_left_is_not_sent ... ok
test edsm::api::tests::a_failed_flight_log_request_never_shows_the_key ... ok
test context::tests::loadgame_supplies_version_without_a_fileheader ... ok
test companion::tests::symbol_stripping ... ok
test context::tests::new_session_fileheader_resets_everything ... ok
test edsm::api::tests::a_flight_log_error_says_what_edsm_said ... ok
test context::tests::partial_location_clears_the_fix ... ok
test edsm::api::tests::the_flight_log_is_read_with_the_commanders_key ... ok
test context::tests::header_and_flags_come_from_fileheader_and_loadgame ... ok
test companion::tests::shipyard_fixture_validates_against_shipyard_v2 ... ok
test companion::tests::outfitting_fixture_validates_against_outfitting_v2 ... ok
test companion::tests::carrier_docking_access_is_renamed ... ok
test edsm::api::tests::http_api_posts_json_and_reads_the_whole_answer ... ok
test companion::tests::market_fixture_validates_against_commodity_v3 ... ok
test companion::tests::schema_rejects_what_the_readme_elides ... ok
test edsm::tests::a_413_batch_is_split_the_same_way ... ok
test edsm::tests::a_key_that_is_still_wrong_shows_its_error_from_the_same_pass ... ok
test edsm::tests::a_list_from_an_earlier_run_is_used_until_it_is_a_day_old ... ok
test edsm::tests::a_single_event_that_keeps_failing_is_given_up_and_later_events_go ... ok
test edsm::tests::another_commanders_events_are_held_back_and_logged_once ... ok
test edsm::tests::events_queued_for_another_commander_are_not_sent_with_this_key ... ok
test edsm::tests::a_corrected_key_sends_what_a_wrong_one_held_back_at_once ... ok
test edsm::tests::once_the_list_is_known_discarded_events_are_not_even_queued ... ok
test edsm::tests::off_or_without_credentials_nothing_is_queued_and_the_queue_is_discarded ... ok
test edsm::tests::a_wrong_commander_or_key_keeps_the_events_for_later ... ok
test edsm::tests::a_request_holds_one_game_session_and_at_most_a_batch ... ok
test edsm::tests::each_event_is_settled_by_its_own_answer ... ok
test edsm::tests::what_counts_as_refused_and_what_is_only_held ... ok
test edsm::tests::fixture_session_requests_exclude_everything_on_the_discard_list ... ok
test edsm::tests::the_startup_replay_is_not_sent ... ok
test edsm::transient::tests::another_commanders_crew_is_a_crew_session_until_it_ends ... ok
test edsm::transient::tests::events_carry_where_the_commander_is_and_in_which_ship ... ok
test edsm::tests::turning_sharing_off_does_not_wake_or_release_anything ... ok
test edsm::transient::tests::proving_ground_and_cqc_are_no_system ... ok
test events::schema_tests::a_run_with_only_other_systems_or_mission_targets_sends_nothing ... ok
test edsm::tests::a_poisoned_event_costs_only_itself ... ok
test events::schema_tests::a_run_of_signals_is_one_message_ended_by_the_next_other_event ... ok
test edsm::tests::the_queue_behind_a_poisoned_event_moves_while_it_waits ... ok
test edsm::tests::a_known_poisoned_event_is_not_sent_back_through_the_halving ... ok
test events::schema_tests::dockinggranted_and_dockingdenied_validate ... ok
test edsm::tests::beta_and_crew_sessions_are_not_sent ... ok
test events::schema_tests::approachsettlement_keeps_latitude_and_longitude_and_adds_the_system ... ok
test events::schema_tests::fcmaterials_journal_comes_from_the_announced_file ... ok
test edsm::tests::without_a_discard_list_nothing_is_sent ... ok
test edsm::tests::outages_do_not_use_up_a_messages_refusals ... ok
test events::sc
```

## Stderr

Bytes: 282
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-af2ff1c537984d03)
     Running tests/documented_limits.rs (target/debug/deps/documented_limits-30eea09462a263b7)
   Doc-tests astromech_eddn
```

