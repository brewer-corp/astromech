---
created_at: "2026-10-17T02:53:59.848363778+00:00"
id: "astromech-kdqa"
evidence_type: "test"
commit: "92488476bbfee48761a98e8011d98a67b4aeca94"
result: "pass"
captured_at: "2026-10-17T02:53:47.494515981+00:00"
command: "cargo test -p astromech-market --test recorded"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mcxc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mcxc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-market --test recorded"
updated_at: "2026-10-17T02:53:59.848805141+00:00"
---

## Summary

cargo test -p astromech-market --test recorded

## Command

```console
cargo test -p astromech-market --test recorded
```

Exit status: 0

## Stdout

Bytes: 1093
Truncated: no

```text

running 15 tests
test demand_caps_keep_a_small_buyer_from_winning_a_big_load ... ok
test the_hold_leaves_out_mission_cargo_stolen_goods_and_limpets ... ok
test ore_sale_with_demand_for_none_of_it_is_still_a_partial_sale ... ok
test ore_sale_is_the_best_proceeds_within_demand_not_the_best_price ... ok
test without_a_position_distances_are_ardents ... ok
test app_search_keeps_the_best_few_with_jump_counts ... ok
test sell_excludes_markets_with_less_demand_than_asked ... ok
test hotspots_pick_the_ring_not_the_body ... ok
test hotspots_are_the_nearest_rings_holding_the_chosen_ore ... ok
test sell_returns_the_best_price_within_the_distance_and_filters ... ok
test several_commodities_in_one_lookup_each_ranked_and_cached ... ok
test whole_load_answer_maximises_proceeds_within_demand_caps ... ok
test buy_returns_the_lowest_price_with_enough_stock ... ok
test per_commodity_answers_match_the_single_commodity_sell_lookup ... ok
test a_full_answer_is_asked_again_with_the_demand_needed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
```

## Stderr

Bytes: 330
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mcxc/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.94s
     Running tests/recorded.rs (target/debug/deps/recorded-708aa94f23523114)
```

