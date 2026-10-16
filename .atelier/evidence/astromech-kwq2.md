---
created_at: "2026-10-16T20:39:26.693170547+00:00"
id: "astromech-kwq2"
evidence_type: "test"
commit: "391bf16e5c12553c6ad88914935f085d755a6c76"
result: "pass"
captured_at: "2026-10-16T20:39:25.362242120+00:00"
command: "cargo test -p astromech-market"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-xqdl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xqdl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded-response lookups: best price within distance and filters; low demand/stock excluded; buy side; several commodities cached; app search with jumps"
updated_at: "2026-10-16T20:39:26.693558331+00:00"
---

## Summary

Recorded-response lookups: best price within distance and filters; low demand/stock excluded; buy side; several commodities cached; app search with jumps

## Command

```console
cargo test -p astromech-market
```

Exit status: 0

## Stdout

Bytes: 847
Truncated: no

```text

running 2 tests
test catalog::tests::catalogue_has_the_mined_goods_and_unique_symbols ... ok
test catalog::tests::find_by_name_alias_or_symbol ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test without_a_position_distances_are_ardents ... ok
test sell_excludes_markets_with_less_demand_than_asked ... ok
test sell_returns_the_best_price_within_the_distance_and_filters ... ok
test app_search_keeps_the_best_few_with_jump_counts ... ok
test buy_returns_the_lowest_price_with_enough_stock ... ok
test several_commodities_in_one_lookup_each_ranked_and_cached ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 391
Truncated: no

```text
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.19s
     Running unittests src/lib.rs (target/debug/deps/astromech_market-642f5789522bc8f7)
     Running tests/recorded.rs (target/debug/deps/recorded-7a53276424543fd4)
   Doc-tests astromech_market
```

