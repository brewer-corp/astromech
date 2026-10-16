---
created_at: "2026-10-16T22:50:34.553864543+00:00"
id: "astromech-2dx8"
evidence_type: "test"
commit: "63e039957666524f4c33bd07fe8bed2051df8e39"
result: "pass"
captured_at: "2026-10-16T22:50:17.697558204+00:00"
command: "bash -c 'cargo test -p astromech-intent --locked && cargo test -p astromech-galaxy --locked && cargo test -p astromech-tools --locked && cargo test -p astromech-core --locked'"
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
title: "After merging trade (zf3s): station and trade voice phrases with one grammar (buy routes to commodity/module/ship by slot), trade intent tests, trade_voice, recorded-response lookups"
updated_at: "2026-10-16T22:50:34.557114785+00:00"
---

## Summary

After merging trade (zf3s): station and trade voice phrases with one grammar (buy routes to commodity/module/ship by slot), trade intent tests, trade_voice, recorded-response lookups

## Command

```console
bash -c 'cargo test -p astromech-intent --locked && cargo test -p astromech-galaxy --locked && cargo test -p astromech-tools --locked && cargo test -p astromech-core --locked'
```

Exit status: 0

## Stdout

Bytes: 11152
Truncated: yes

```text

running 15 tests
test tests::class_and_rating_said_apart_stay_together ... ok
test tests::normalisation ... ok
test tests::noise_only_transcripts ... ok
test tests::thresholds_are_the_documented_ones ... ok
test tests::free_words_ride_along_in_the_slot_text ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::s9_mic_misses ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::multi_word_slots_resolve_to_their_value ... ok
test tests::real_game_transcripts ... ok
test tests::exact_phrases_execute ... ok
test tests::v1_t1_and_status_phrases_execute ... ok
test tests::trade_phrases_fill_the_commodity_slot ... ok
test tests::misheard_commodities_ask ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 29 tests
test filters::tests::keeps_applies_each_filter ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test remote::tests::parse_and_match_fixtures ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test stations::tests::service_ids_round_trip ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test outfitting::tests::every_catalogue_name_is_one_spansh_uses ... ok
test stations::tests::modules_spansh_names_differently_are_asked_for_by_every_name ... ok
test tests::farming_notes_name_known_lookups ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test tests::here_from_journals ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 6 tests
test reinforcement_systems_are_the_pledged_powers_nearest_first ... ok
test hits_are_in_distance_order_whatever_order_spansh_answers_in ... ok
test acquisition_systems_carry_the_powers_progress ... ok
test the_age_filter_applies_and_the_answer_is_cached ... ok
test answers_that_ignore_the_filter_are_dropped_with_a_warning ... ok
test undermining_systems_belong_to_other_powers ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test tests::commands_map ... ok
test tests::flight_assist_flag_is_inverted ... ok
test tests::gear_state_rules ... ok
test lookup::tests::service_slot_values_name_every_service ... ok
test tests::silent_running_on_asks ... ok
test tests::other_toggles ... ok
test tests::status_answers ... ok
test tests::screens_and_throttle ... ok
test tests::system_powerplay_is_answered_from_the_last_jump ... ok
test tests::powerplay_rank_is_answered_from_the_jou
```

## Stderr

Bytes: 2590
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-journal)
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-data)
   Compiling getrandom v0.4.3
   Compiling tempfile v3.27.0
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-powerplay)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-materials)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.80s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-62ff62d6547e298f)
     Running tests/powerplay.rs (target/debug/deps/powerplay-1f8525a1f0c45e0c)
   Doc-tests astromech_galaxy
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-data)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-gamestate)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.59s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-9e25cb1f80fac25d)
   Doc-tests astromech_tools
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.66s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-57838e1ca55eeb18)
     Running unittests src/main.rs (target/debug/deps/astromech_core-af30d4660710c82b)
     Running tests/gear_down.rs (target/debug/deps/gear_down-5de8cf450394c37a)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-c9296b3d632a36af)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-897a36bdc7f8e0c6)
   Doc-tests astromech_core
```

