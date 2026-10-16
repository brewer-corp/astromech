---
created_at: "2026-10-16T20:49:21.415628946+00:00"
id: "astromech-7drd"
evidence_type: "test"
commit: "b7f8ebf10ba90a43749644253f10789ef81d8c5f"
result: "pass"
captured_at: "2026-10-16T20:49:16.799710061+00:00"
command: "cargo test -p astromech-market -p astromech-journal"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-sd56"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-sd56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded-response cargo plan: fixture hold of three mined commodities; whole-load answer equals the independent maximum of min(carried, demand) x price over every market (hand-worked Good Landing 183,674,700); demand caps (1,000 t LTDs, Sabine Point takes 483); per-commodity answers equal the single-commodity sell lookup with demand for the load; full Ardent answer is asked again with minVolume; mission/stolen/limpets left out"
updated_at: "2026-10-16T20:49:21.416026078+00:00"
---

## Summary

Recorded-response cargo plan: fixture hold of three mined commodities; whole-load answer equals the independent maximum of min(carried, demand) x price over every market (hand-worked Good Landing 183,674,700); demand caps (1,000 t LTDs, Sabine Point takes 483); per-commodity answers equal the single-commodity sell lookup with demand for the load; full Ardent answer is asked again with minVolume; mission/stolen/limpets left out

## Command

```console
cargo test -p astromech-market -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 2948
Truncated: no

```text

running 7 tests
test tests::cargo_and_route_files ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::positions_and_engineer_events ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::loadout_modules_and_fleet_events ... ok
test tests::known_and_unknown_events ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 28 tests
test cargo_inventory ... ok
test bounty_and_voucher ... ok
test carrier_family ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test colonisation_family ... ok
test exobiology_events ... ok
test exploration_sales ... ok
test codex_settlement_and_beacon ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test docking_granted_and_denied ... ok
test market_buy_and_sell ... ok
test jump_planning_events ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test saa_events ... ok
test mining ... ok
test combat_events ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test rank_progress_reputation ... ok
test supercruise_transitions ... ok
test powerplay_family ... ok
test unknown_events_still_fall_through_to_other ... ok
test scan_body_and_barycentre ... ok
test ship_locker ... ok
test fuel_chat_and_surface_events ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test missions ... ok
test fss_events ... ok
test statistics_flattens_numeric_counters ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 2 tests
test catalog::tests::catalogue_has_the_mined_goods_and_unique_symbols ... ok
test catalog::tests::find_by_name_alias_or_symbol ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
test the_hold_leaves_out_mission_cargo_stolen_goods_and_limpets ... ok
test sell_excludes_markets_with_less_demand_than_asked ... ok
test without_a_position_distances_are_ardents ... ok
test app_search_keeps_the_best_few_with_jump_counts ... ok
test demand_caps_keep_a_small_buyer_from_winning_a_big_load ... ok
test buy_returns_the_lowest_price_with_enough_stock ... ok
test several_commodities_in_one_lookup_each_ranked_and_cached ... ok
test whole_load_answer_maximises_proceeds_within_demand_caps ... ok
test per_commodity_answers_match_the_single_commodity_sell_lookup ... ok
test sell_returns_the_best_price_within_the_distance_and_filters ... ok
test a_full_answer_is_asked_again_with_the_demand_needed ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 832
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-galaxy)
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-journal)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.22s
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-1440aa04ea0996e8)
     Running tests/events.rs (target/debug/deps/events-f2557607e9be31ed)
     Running unittests src/lib.rs (target/debug/deps/astromech_market-aa81156dd90e10bc)
     Running tests/recorded.rs (target/debug/deps/recorded-b9a8f8afadfc6038)
   Doc-tests astromech_journal
   Doc-tests astromech_market
```

