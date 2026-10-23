---
created_at: "2026-10-22T23:05:02.897891848+00:00"
id: "astromech-62fg"
evidence_type: "test"
commit: "eb5b71b2fdbffdd1dadc903364da4756c4b78d2d"
result: "pass"
captured_at: "2026-10-22T23:04:55.842213229+00:00"
command: "env CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy"
exit_status: "0"
agent_identity: "claude-implementer-6bjl"
target:
  kind: "issue"
  id: "astromech-6bjl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6bjl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-galaxy tests pass: synthetic fields vs a brute-force reference (chain, detour, gap, start/destination on a neutron star, k=4 and 6, 200 random fields), Spansh's recorded routes (per-leg model and no worse on its own stars), plot() names and labels, names refused after the file is replaced, the guided search on a dense field; each seen failing under 13 mutations"
updated_at: "2026-10-22T23:05:02.898173673+00:00"
---

## Summary

astromech-galaxy tests pass: synthetic fields vs a brute-force reference (chain, detour, gap, start/destination on a neutron star, k=4 and 6, 200 random fields), Spansh's recorded routes (per-leg model and no worse on its own stars), plot() names and labels, names refused after the file is replaced, the guided search on a dense field; each seen failing under 13 mutations

## Command

```console
env CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 7501
Truncated: yes

```text

running 46 tests
test filters::tests::keeps_applies_each_filter ... ok
test filters::tests::newer_hulls_have_their_checked_pads ... ok
test filters::tests::updated_since_rounds_down_to_the_hour ... ok
test neutron_plot::tests::drops_only_the_stops_that_save_no_jumps ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test remote::tests::parse_and_match_fixtures ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test outfitting::tests::the_catalogue_is_every_module_spansh_lists ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test stations::tests::a_pre_engineered_module_and_its_base_module_are_told_apart ... ok
test stations::tests::service_ids_round_trip ... ok
test stations::tests::a_mount_none_of_the_page_sells_says_so_not_that_the_filter_changed ... ok
test stations::tests::modules_spansh_names_differently_are_asked_for_by_every_name ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::armour_is_found_by_its_ship_symbols ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test tests::farming_notes_name_known_lookups ... ok
test tests::here_from_journals ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test tests::hge_score_prefers_a_billion_people ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test tests::hge_lookups_rank_by_population_and_distance ... ok
test tests::arrival_faction_states_replace_older_spansh_ones ... ok
test tests::a_strict_age_still_finds_fresh_systems_farther_out ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::the_engineering_watch_list_has_every_reader_s_lines ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test visits::tests::a_carrier_jump_is_a_visit ... ok
test visits::tests::newest_visit_per_system_wins ... ok
test visits::tests::states_take_spansh_spelling ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test filters::tests::a_search_resolves_the_pad_from_the_current_ship_not_the_filters_sent ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test filters::tests::the_remembered_filters_follow_the_ship_flown ... ok
test tests::hge_lookups_also_ask_for_the_nearest_billion_people_systems ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 8 tests
test an_ignored_population_filter_gives_nothing_and_a_warning ... ok
test an_area_where_everything_is_claimed_gives_nothing_without_a_warning ... ok
test the_age_filter_and_the_cache_apply ... ok
test a_truncated_populated_list_counts_what_it_couldnt_check ... ok
test a_recorded_answer_gives_only_unclaimed_systems_within_15_ly_of_a_populated_one ... ok
test spansh_distances_alone_skip_the_populated_search ... ok
test the_data_age_is_sent_with_the_unpopulated_search ... ok
test bodies_of_interest_narrow_the_list ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 7 tests
test a_cell_with_no_stars_is_empty ... ok
test indexes_stars_given_in_any_order ... ok
test refuses_other_files ... ok
test finds_the_cells_in_a_box ... ok
test drops_the_systems_without_a
```

## Stderr

Bytes: 839
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-3d94b29ccba1d3a0)
     Running tests/colonies.rs (target/debug/deps/colonies-8044f9c2f0aaf4d1)
     Running tests/neutron.rs (target/debug/deps/neutron-48b2a8bf6c20b67e)
     Running tests/neutron_download.rs (target/debug/deps/neutron_download-ae430d75e633b88b)
     Running tests/neutron_plot.rs (target/debug/deps/neutron_plot-f40c669f7b797176)
     Running tests/plot.rs (target/debug/deps/plot-fc8fa60ce1dcd988)
     Running tests/powerplay.rs (target/debug/deps/powerplay-a82104390b26bb43)
     Running tests/powerplay_logistics.rs (target/debug/deps/powerplay_logistics-0b35358b5b4b65f9)
     Running tests/sites.rs (target/debug/deps/sites-08459f810c061b77)
   Doc-tests astromech_galaxy
```

