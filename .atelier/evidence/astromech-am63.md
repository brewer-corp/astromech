---
created_at: "2026-10-17T17:42:19.874753186+00:00"
id: "astromech-am63"
evidence_type: "test"
commit: "d883b7ccd4ec7ebc6746f092262c1ca092144eeb"
result: "pass"
captured_at: "2026-10-17T17:42:18.588346428+00:00"
command: "cargo test -p astromech-galaxy"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-mf5e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mf5e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-galaxy"
updated_at: "2026-10-17T17:42:19.875012280+00:00"
---

## Summary

cargo test -p astromech-galaxy

## Command

```console
cargo test -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 5198
Truncated: yes

```text

running 43 tests
test filters::tests::newer_hulls_have_their_checked_pads ... ok
test filters::tests::keeps_applies_each_filter ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test filters::tests::updated_since_rounds_down_to_the_hour ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test remote::tests::parse_and_match_fixtures ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test outfitting::tests::the_catalogue_is_every_module_spansh_lists ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test stations::tests::armour_is_found_by_its_ship_symbols ... ok
test stations::tests::a_mount_none_of_the_page_sells_says_so_not_that_the_filter_changed ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test stations::tests::service_ids_round_trip ... ok
test stations::tests::a_pre_engineered_module_and_its_base_module_are_told_apart ... ok
test stations::tests::modules_spansh_names_differently_are_asked_for_by_every_name ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test tests::here_from_journals ... ok
test tests::farming_notes_name_known_lookups ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test tests::hge_score_prefers_a_billion_people ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test tests::hge_lookups_rank_by_population_and_distance ... ok
test tests::arrival_faction_states_replace_older_spansh_ones ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test store::tests::seed_insert_cover_cache ... ok
test visits::tests::states_take_spansh_spelling ... ok
test visits::tests::newest_visit_per_system_wins ... ok
test tests::a_strict_age_still_finds_fresh_systems_farther_out ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test filters::tests::a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test filters::tests::the_remembered_filters_follow_the_ship_flown ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test tests::hge_lookups_also_ask_for_the_nearest_billion_people_systems ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 43 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 8 tests
test an_area_where_everything_is_claimed_gives_nothing_without_a_warning ... ok
test an_ignored_population_filter_gives_nothing_and_a_warning ... ok
test spansh_distances_alone_skip_the_populated_search ... ok
test a_recorded_answer_gives_only_unclaimed_systems_within_15_ly_of_a_populated_one ... ok
test the_age_filter_and_the_cache_apply ... ok
test a_truncated_populated_list_counts_what_it_couldnt_check ... ok
test the_data_age_is_sent_with_the_unpopulated_search ... ok
test bodies_of_interest_narrow_the_list ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 6 tests
test answers_that_ignore_the_filter_are_dropped_with_a_warning ... ok
test reinforcement_systems_are_the_pledged_powers_nearest_first ... ok
test hits_are_in_distance_order_whatever_order_spansh_answers_in ... ok
test acquisition_systems_carry_the_powers_progress ... ok
test undermining_systems_belong_to_other_powers ... ok
test the_age_filter_applies_and_the_answer_is_cached ... ok
```

## Stderr

Bytes: 517
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-ad39483b093d6fab)
     Running tests/colonies.rs (target/debug/deps/colonies-3832fb779d8b3810)
     Running tests/powerplay.rs (target/debug/deps/powerplay-80600fd9fc400a95)
     Running tests/powerplay_logistics.rs (target/debug/deps/powerplay_logistics-f2b188312b6e7b3e)
     Running tests/sites.rs (target/debug/deps/sites-ba99a6d8e43bff49)
   Doc-tests astromech_galaxy
```

