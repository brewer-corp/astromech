---
created_at: "2026-10-17T02:49:12.477533276+00:00"
id: "astromech-w00w"
evidence_type: "test"
commit: "81ec4a045bd1bebd95453b4f41c32e9d39f396df"
result: "pass"
captured_at: "2026-10-17T02:49:11.717931636+00:00"
command: "cargo test -p astromech-market --test here"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-qw2m"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qw2m"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes: market comparison tests incl. 14-kind hold makes exactly 12 requests (R1)"
updated_at: "2026-10-17T02:49:12.477807217+00:00"
---

## Summary

After review fixes: market comparison tests incl. 14-kind hold makes exactly 12 requests (R1)

## Command

```console
cargo test -p astromech-market --test here
```

Exit status: 0

## Stdout

Bytes: 563
Truncated: no

```text

running 7 tests
test market_symbols_are_ardents ... ok
test lookups_are_capped_carried_first_then_the_goods_furthest_below_average ... ok
test a_hold_of_more_kinds_than_the_cap_asks_no_more_than_the_cap ... ok
test the_hold_and_the_goods_for_sale_against_the_best_nearby_prices ... ok
test the_lookups_filters_apply_to_the_nearby_markets ... ok
test carried_goods_need_a_market_with_demand_for_all_of_them ... ok
test a_second_look_is_answered_from_the_cache ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Stderr

Bytes: 262
Truncated: no

```text
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-qw2m/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running tests/here.rs (target/debug/deps/here-3f02193e50d153c0)
```

