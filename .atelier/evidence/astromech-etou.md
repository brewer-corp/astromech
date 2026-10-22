---
created_at: "2026-10-22T20:47:30.756577926+00:00"
id: "astromech-etou"
evidence_type: "test"
commit: "141d4e4de123567f226682b21e1df86fa00cfd1d"
result: "pass"
captured_at: "2026-10-22T20:47:27.416220134+00:00"
command: "env CARGO_BUILD_JOBS=4 cargo test -p astromech-data -p astromech-galaxy"
exit_status: "0"
agent_identity: "claude-implementer-vlsz"
target:
  kind: "issue"
  id: "astromech-vlsz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-vlsz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "NeutronRoute and ExactRoute forms carry supercharge_multiplier (6 for a Mk II drive; exact 4 echoed back by the recorded Spansh job), and astromech_galaxy::plot fills it from the ship's Fsd for both plotters"
updated_at: "2026-10-22T20:47:30.756926776+00:00"
---

## Summary

NeutronRoute and ExactRoute forms carry supercharge_multiplier (6 for a Mk II drive; exact 4 echoed back by the recorded Spansh job), and astromech_galaxy::plot fills it from the ship's Fsd for both plotters

## Command

```console
env CARGO_BUILD_JOBS=4 cargo test -p astromech-data -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 7656
Truncated: yes

```text

running 7 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test transport::tests::segments_and_cache_keys ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok
test cache::tests::the_shared_file_waits_for_the_other_process ... ok
test cache::tests::a_v015_table_without_expiry_times_is_replaced ... ok
test cache::tests::two_openers_of_a_v015_file_both_get_the_file_backed_cache ... ok
test cache::tests::two_openers_of_a_fresh_file_both_get_the_file_backed_cache ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s


running 21 tests
test http_errors_are_plain_words ... ok
test a_schema_error_is_one_spoken_sentence_without_the_serde_detail ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test no_network_gives_a_plain_words_error ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test a_broken_cache_still_returns_the_network_answer ... ok
test spansh_route_job_round_trip ... ok
test spansh_exact_route_failures_are_plain_words ... ok
test spansh_system_names_autocomplete ... ok
test spansh_exact_route_posts_the_drive_and_polls_until_done ... ok
test uncached_calls_always_ask ... ok
test spansh_trade_route_polls_until_the_job_is_done ... ok
test spansh_neutron_route_polls_until_done ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok
test expired_rows_are_pruned_so_the_cache_holds_only_live_answers ... ok
test spansh_searches_parse_recorded_answers ... ok
test spansh_trade_route_errors_are_plain_words ... ok
test a_cache_locked_by_the_other_process_waits_for_it_then_caches ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s


running 45 tests
test filters::tests::keeps_applies_each_filter ... ok
test filters::tests::newer_hulls_have_their_checked_pads ... ok
test filters::tests::updated_since_rounds_down_to_the_hour ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test remote::tests::parse_and_match_fixtures ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test outfitting::tests::the_catalogue_is_every_module_spansh_lists ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test stations::tests::a_mount_none_of_the_page_sells_says_so_not_that_the_filter_changed ... ok
test stations::tests::armour_is_found_by_its_ship_symbols ... ok
test stations::tests::service_ids_round_trip ... ok
test stations::tests::a_pre_engineered_module_and_its_base_module_are_told_apart ... ok
test stations::tests::modules_spansh_names_differently_are_asked_for_by_every_name ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test tests::farming_notes_name_known_lookups ... ok
test tests::here_from_journals ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test tests::hge_score_prefers_a_billion_people ... ok
test tests::hge_lookups_rank_by_population_and_distance ... ok
test tests::arrival_faction_states_replace_older_spansh_ones ... ok
test tests::the_engineering_watch_list_has_every_reader_s_lines ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test tests::nearby_ranks_cache
```

## Stderr

Bytes: 837
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-4eef9b803ee120b6)
     Running tests/recorded.rs (target/debug/deps/recorded-a88248fb3ac37a13)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-e848ade3cef4ca40)
     Running tests/colonies.rs (target/debug/deps/colonies-8da3dadaeac9a6de)
     Running tests/plot.rs (target/debug/deps/plot-64b3010cdd832c24)
     Running tests/powerplay.rs (target/debug/deps/powerplay-00251dfa0f3ed374)
     Running tests/powerplay_logistics.rs (target/debug/deps/powerplay_logistics-88a85c1c01a3bbea)
     Running tests/sites.rs (target/debug/deps/sites-72ac60f1f7952a8f)
   Doc-tests astromech_data
   Doc-tests astromech_galaxy
```

