---
created_at: "2026-10-22T21:30:46.584069313+00:00"
id: "astromech-rvx0"
evidence_type: "test"
commit: "dfcb12fd136d61438891084e3e8672dc52cabd6c"
result: "pass"
captured_at: "2026-10-22T21:30:40.680100677+00:00"
command: "cargo test -p astromech-powerplay -p astromech-galaxy --lib --tests"
exit_status: "0"
agent_identity: "claude-implementer-fdh3"
target:
  kind: "issue"
  id: "astromech-xvov"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xvov"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Follower reads only appended lines and equals a full replay (partial lines, new file, replay when an older file goes); first login of a run without Powerplay clears the pledge, relog doesn't; contested state named; undermine re-check requires a controlled state"
updated_at: "2026-10-22T21:30:46.584418875+00:00"
---

## Summary

Follower reads only appended lines and equals a full replay (partial lines, new file, replay when an older file goes); first login of a run without Powerplay clears the pledge, relog doesn't; contested state named; undermine re-check requires a controlled state

## Command

```console
cargo test -p astromech-powerplay -p astromech-galaxy --lib --tests
```

Exit status: 0

## Stdout

Bytes: 7168
Truncated: yes

```text

running 46 tests
test filters::tests::keeps_applies_each_filter ... ok
test filters::tests::newer_hulls_have_their_checked_pads ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test filters::tests::updated_since_rounds_down_to_the_hour ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test remote::tests::parse_and_match_fixtures ... ok
test powerplay::tests::undermine_recheck_checks_the_state ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test stations::tests::armour_is_found_by_its_ship_symbols ... ok
test stations::tests::a_mount_none_of_the_page_sells_says_so_not_that_the_filter_changed ... ok
test stations::tests::a_pre_engineered_module_and_its_base_module_are_told_apart ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test outfitting::tests::the_catalogue_is_every_module_spansh_lists ... ok
test stations::tests::service_ids_round_trip ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test stations::tests::modules_spansh_names_differently_are_asked_for_by_every_name ... ok
test tests::farming_notes_name_known_lookups ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test tests::here_from_journals ... ok
test tests::arrival_faction_states_replace_older_spansh_ones ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test tests::hge_lookups_rank_by_population_and_distance ... ok
test tests::hge_score_prefers_a_billion_people ... ok
test tests::the_engineering_watch_list_has_every_reader_s_lines ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test visits::tests::a_carrier_jump_is_a_visit ... ok
test visits::tests::newest_visit_per_system_wins ... ok
test visits::tests::states_take_spansh_spelling ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test filters::tests::the_remembered_filters_follow_the_ship_flown ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::a_strict_age_still_finds_fresh_systems_farther_out ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test tests::hge_lookups_also_ask_for_the_nearest_billion_people_systems ... ok
test filters::tests::a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s


running 8 tests
test an_ignored_population_filter_gives_nothing_and_a_warning ... ok
test an_area_where_everything_is_claimed_gives_nothing_without_a_warning ... ok
test spansh_distances_alone_skip_the_populated_search ... ok
test a_truncated_populated_list_counts_what_it_couldnt_check ... ok
test a_recorded_answer_gives_only_unclaimed_systems_within_15_ly_of_a_populated_one ... ok
test the_data_age_is_sent_with_the_unpopulated_search ... ok
test the_age_filter_and_the_cache_apply ... ok
test bodies_of_interest_narrow_the_list ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 4 tests
test the_current_ship_gives_the_range_and_drive ... ok
test a_neutron_route_stops_at_the_neutron_stars ... ok
test an_exact_route_has_every_jump_and_its_refuel_stops ... ok
test plot_failures_are_
```

## Stderr

Bytes: 1132
Truncated: no

```text
   Compiling astromech-powerplay v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-fdh3/crates/astromech-powerplay)
   Compiling astromech-galaxy v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-fdh3/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.38s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-e848ade3cef4ca40)
     Running tests/colonies.rs (target/debug/deps/colonies-8da3dadaeac9a6de)
     Running tests/plot.rs (target/debug/deps/plot-64b3010cdd832c24)
     Running tests/powerplay.rs (target/debug/deps/powerplay-00251dfa0f3ed374)
     Running tests/powerplay_logistics.rs (target/debug/deps/powerplay_logistics-88a85c1c01a3bbea)
     Running tests/sites.rs (target/debug/deps/sites-72ac60f1f7952a8f)
     Running unittests src/lib.rs (target/debug/deps/astromech_powerplay-cc615d4bf85cd621)
     Running tests/activities.rs (target/debug/deps/activities-732735abe3cf335a)
     Running tests/arrival.rs (target/debug/deps/arrival-13e892b1984c9b5e)
     Running tests/standing.rs (target/debug/deps/standing-4b3b8e47f549d8c7)
```

