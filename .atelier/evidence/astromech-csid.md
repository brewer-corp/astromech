---
created_at: "2026-10-17T18:52:36.448385306+00:00"
id: "astromech-csid"
evidence_type: "test"
commit: "cfd6901fe4c44b48399bc05689b2ea09e1eb4c25"
result: "pass"
captured_at: "2026-10-17T18:52:28.110360189+00:00"
command: "cargo test -p astromech-materials -p astromech-galaxy"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ldiz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ldiz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-materials -p astromech-galaxy"
updated_at: "2026-10-17T18:52:36.448690546+00:00"
---

## Summary

cargo test -p astromech-materials -p astromech-galaxy

## Command

```console
cargo test -p astromech-materials -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 6696
Truncated: yes

```text

running 45 tests
test filters::tests::keeps_applies_each_filter ... ok
test filters::tests::newer_hulls_have_their_checked_pads ... ok
test filters::tests::updated_since_rounds_down_to_the_hour ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test remote::tests::parse_and_match_fixtures ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test outfitting::tests::the_catalogue_is_every_module_spansh_lists ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test stations::tests::a_mount_none_of_the_page_sells_says_so_not_that_the_filter_changed ... ok
test stations::tests::service_ids_round_trip ... ok
test stations::tests::a_pre_engineered_module_and_its_base_module_are_told_apart ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test stations::tests::armour_is_found_by_its_ship_symbols ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::modules_spansh_names_differently_are_asked_for_by_every_name ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test tests::farming_notes_name_known_lookups ... ok
test tests::here_from_journals ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test tests::hge_score_prefers_a_billion_people ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test tests::hge_lookups_rank_by_population_and_distance ... ok
test tests::arrival_faction_states_replace_older_spansh_ones ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::a_strict_age_still_finds_fresh_systems_farther_out ... ok
test tests::the_engineering_watch_list_has_every_reader_s_lines ... ok
test visits::tests::newest_visit_per_system_wins ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test visits::tests::a_carrier_jump_is_a_visit ... ok
test visits::tests::states_take_spansh_spelling ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test tests::hge_lookups_also_ask_for_the_nearest_billion_people_systems ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test filters::tests::the_remembered_filters_follow_the_ship_flown ... ok
test filters::tests::a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 8 tests
test an_ignored_population_filter_gives_nothing_and_a_warning ... ok
test an_area_where_everything_is_claimed_gives_nothing_without_a_warning ... ok
test spansh_distances_alone_skip_the_populated_search ... ok
test a_recorded_answer_gives_only_unclaimed_systems_within_15_ly_of_a_populated_one ... ok
test a_truncated_populated_list_counts_what_it_couldnt_check ... ok
test the_age_filter_and_the_cache_apply ... ok
test the_data_age_is_sent_with_the_unpopulated_search ... ok
test bodies_of_interest_narrow_the_list ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 6 tests
test answers_that_ignore_the_filter_are_dropped_with_a_warning ... ok
test acquisition_systems_carry_the_powers_progress ... ok
test undermining_systems_belong_to_other_powers ... ok
test reinforcement_systems_are_the_pledged_powers_nearest_first ... ok
test hi
```

## Stderr

Bytes: 1099
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-materials)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-bugs-live/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.63s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-925b5d51d2936649)
     Running tests/colonies.rs (target/debug/deps/colonies-b910536d9f8c39d0)
     Running tests/powerplay.rs (target/debug/deps/powerplay-70637a83d75cd8ae)
     Running tests/powerplay_logistics.rs (target/debug/deps/powerplay_logistics-794460c59f1182e8)
     Running tests/sites.rs (target/debug/deps/sites-6ab30a31df7cf4d9)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-77abff06b0dd81eb)
     Running tests/ledger_events.rs (target/debug/deps/ledger_events-08d941b2136e6556)
     Running tests/live_journal.rs (target/debug/deps/live_journal-6d0eb9c408465b14)
   Doc-tests astromech_galaxy
   Doc-tests astromech_materials
```

