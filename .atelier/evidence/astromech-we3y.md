---
created_at: "2026-10-22T21:43:55.571823802+00:00"
id: "astromech-we3y"
evidence_type: "test"
commit: "b4e118b5345dafd978718e915eb4f40562203b41"
result: "pass"
captured_at: "2026-10-22T21:41:01.270992962+00:00"
command: "bash -c 'python3 scripts/check-deps.py && ! cargo tree -p astromech-engineering -e normal | grep -E '\"'\"'astromech-data|rusqlite|ureq'\"'\"' && CARGO_BUILD_JOBS=4 cargo test -p astromech-proto -p astromech-data -p astromech-engineering'"
exit_status: "0"
agent_identity: "claude-implementer-0k88"
target:
  kind: "issue"
  id: "astromech-zvs9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zvs9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "check-deps passes; astromech-engineering's normal dependency tree has no astromech-data, rusqlite or ureq; proto, data and engineering tests pass with Clock in astromech-proto"
updated_at: "2026-10-22T21:43:55.572426986+00:00"
---

## Summary

check-deps passes; astromech-engineering's normal dependency tree has no astromech-data, rusqlite or ureq; proto, data and engineering tests pass with Clock in astromech-proto

## Command

```console
bash -c 'python3 scripts/check-deps.py && ! cargo tree -p astromech-engineering -e normal | grep -E '"'"'astromech-data|rusqlite|ureq'"'"' && CARGO_BUILD_JOBS=4 cargo test -p astromech-proto -p astromech-data -p astromech-engineering'
```

Exit status: 0

## Stdout

Bytes: 5078
Truncated: yes

```text
dependency rules OK (29 workspace crates)

running 7 tests
test spansh::tests::search_body_uses_verified_shapes ... ok
test transport::tests::segments_and_cache_keys ... ok
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test cache::tests::the_shared_file_waits_for_the_other_process ... ok
test cache::tests::a_v015_table_without_expiry_times_is_replaced ... ok
test cache::tests::two_openers_of_a_v015_file_both_get_the_file_backed_cache ... ok
test cache::tests::two_openers_of_a_fresh_file_both_get_the_file_backed_cache ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s


running 21 tests
test ardent_endpoints_parse_recorded_answers ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test http_errors_are_plain_words ... ok
test a_schema_error_is_one_spoken_sentence_without_the_serde_detail ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test a_broken_cache_still_returns_the_network_answer ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test no_network_gives_a_plain_words_error ... ok
test spansh_exact_route_failures_are_plain_words ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test spansh_route_job_round_trip ... ok
test spansh_exact_route_posts_the_drive_and_polls_until_done ... ok
test spansh_trade_route_polls_until_the_job_is_done ... ok
test expired_rows_are_pruned_so_the_cache_holds_only_live_answers ... ok
test spansh_system_names_autocomplete ... ok
test uncached_calls_always_ask ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok
test spansh_neutron_route_polls_until_done ... ok
test spansh_searches_parse_recorded_answers ... ok
test spansh_trade_route_errors_are_plain_words ... ok
test a_cache_locked_by_the_other_process_waits_for_it_then_caches ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s


running 35 tests
test links::tests::edsy_round_trips_the_loadout ... ok
test links::tests::coriolis_round_trips_the_loadout ... ok
test links::tests::only_our_links_are_openable ... ok
test links::tests::urls_snapshot ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test tests::a_shipyard_reopened_mid_transfer_keeps_the_arrival ... ok
test tests::a_refreshed_module_list_keeps_a_transfer_under_way ... ok
test tests::fetched_module_is_in_transit_until_its_transfer_time_passes ... ok
test tests::a_shipyard_listing_a_ship_at_a_station_ends_its_transfer ... ok
test tests::a_swapped_out_module_keeps_its_engineering ... ok
test tests::engineer_grades_are_per_module ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::planned_modules_follow_the_plan_like_the_game ... ok
test tests::remote_sale_and_fetch_follow_the_storage_slot ... ok
test tests::mass_store_and_buy_keep_the_stored_list_current ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::trader_rates ... ok
test tests::stored_modules_follow_store_retrieve_and_remote_sale ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::transferred_ship_arrives_after_its_transfer_time ... ok
test tests::a_journal_folder_replays_the_stored_events ... ok
test tests::transfer_marks_a_ship_in_transit_to_where_we_are ... ok
test tests::crafts_replace_the_modules_modifiers ... ok
test links::tests::large_loadout_urls_stay_openable_and_short ... ok
test unlocks::tests::every_ship_and_suit_engineer_has_unlock_data ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::trip_skips_engineers_capped_below_the_grade_for_this_module ... ok
test tests::catalogue_materials_resolve ... ok
test tests::fixture_fleet_lists_every_ship_and_stored_module ... ok
test tests::grades_remaining_and_shopping_list ... ok
test unlocks::tests::tree_marks_progress_and_what_is_left ... ok
test tests::fleet_from_history ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::trip_in_person_stops_and_remote_note ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok

test resu
```

## Stderr

Bytes: 865
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-engineering v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-0k88/crates/astromech-engineering)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.52s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-7266ab57fe45b2af)
     Running tests/recorded.rs (target/debug/deps/recorded-0edec80cca39a13d)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-6841940e56d22ef5)
     Running tests/live_journal.rs (target/debug/deps/live_journal-160e51091c614ca1)
     Running tests/transfers.rs (target/debug/deps/transfers-a441cf7c420bce5a)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-b4fc05838331bf3c)
   Doc-tests astromech_data
   Doc-tests astromech_engineering
   Doc-tests astromech_proto
```

