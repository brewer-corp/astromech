---
created_at: "2026-10-16T21:06:14.538479501+00:00"
id: "astromech-ztdz"
evidence_type: "test"
commit: "e77e92d8f752358f8cf0a288e6290dd964c68958"
result: "pass"
captured_at: "2026-10-16T21:06:12.802563886+00:00"
command: "bash -c 'cargo test -p astromech-galaxy --locked && cargo test -p astromech-tools --locked && cargo test -p astromech-intent --locked && cargo test -p astromech-core --locked lookups'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-m30a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-m30a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "All station lookups against recorded Spansh responses and the voice phrase tests (services, modules, ships), at the branch head"
updated_at: "2026-10-16T21:06:14.538962272+00:00"
---

## Summary

All station lookups against recorded Spansh responses and the voice phrase tests (services, modules, ships), at the branch head

## Command

```console
bash -c 'cargo test -p astromech-galaxy --locked && cargo test -p astromech-tools --locked && cargo test -p astromech-intent --locked && cargo test -p astromech-core --locked lookups'
```

Exit status: 0

## Stdout

Bytes: 4394
Truncated: yes

```text

running 26 tests
test filters::tests::keeps_applies_each_filter ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test stations::tests::service_ids_round_trip ... ok
test remote::tests::parse_and_match_fixtures ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test tests::here_from_journals ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test tests::farming_notes_name_known_lookups ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 12 tests
test tests::other_toggles ... ok
test tests::status_answers ... ok
test tests::commands_map ... ok
test tests::gear_state_rules ... ok
test lookup::tests::service_slot_values_name_every_service ... ok
test tests::flight_assist_flag_is_inverted ... ok
test tests::screens_and_throttle ... ok
test tests::silent_running_on_asks ... ok
test lookup::tests::module_lookups_read_class_rating_and_mount_from_the_words ... ok
test lookup::tests::module_answer_says_the_price ... ok
test lookup::tests::ship_answer_says_the_price ... ok
test lookup::tests::spoken_answers_for_recorded_responses ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 13 tests
test tests::class_and_rating_said_apart_stay_together ... ok
test tests::noise_only_transcripts ... ok
test tests::normalisation ... ok
test tests::thresholds_are_the_documented_ones ... ok
test tests::free_words_ride_along_in_the_slot_text ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::s9_mic_misses ... ok
test tests::multi_word_slots_resolve_to_their_value ... ok
test tests::real_game_transcripts ... ok
test tests::exact_phrases_execute ... ok
test tests::v1_t1_and_status_phrases_execute ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test tester::tests::lookups_say_what_they_would_look_up ... ok
test lookups::tests::ship_phrases_resolve_to_lookups ... ok
test lookups::tests::module_phrases_resolve_to_lookups ... ok
test lookups::tests::service_phrases_resolve_to_lookups ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.76s


running 0 tests

test result: ok. 0 passe
```

## Stderr

Bytes: 1102
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-d5a668c68e3f5656)
   Doc-tests astromech_galaxy
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-604fc48aee6d0e60)
   Doc-tests astromech_tools
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-9ead6ca7f9b9a7b4)
     Running unittests src/main.rs (target/debug/deps/astromech_core-1620298b589ad367)
     Running tests/gear_down.rs (target/debug/deps/gear_down-f08141c738926b64)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-dff127e51d0f749e)
```

