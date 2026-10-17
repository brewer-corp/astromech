---
created_at: "2026-10-17T02:42:01.185581667+00:00"
id: "astromech-iw9b"
evidence_type: "test"
commit: "c82ab98081dafc858e0ad7b50b84a0de320cba28"
result: "pass"
captured_at: "2026-10-17T02:41:54.940031788+00:00"
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
title: "Done-when: fixture Market.json + recorded Ardent nearby prices give the expected differences (gold +3,921/t, tritium +52,506/t, silver +3,321/t, hydrogen fuel +80/t; filters, demand cap, lookup cap, cache)"
updated_at: "2026-10-17T02:42:01.186032498+00:00"
---

## Summary

Done-when: fixture Market.json + recorded Ardent nearby prices give the expected differences (gold +3,921/t, tritium +52,506/t, silver +3,321/t, hydrogen fuel +80/t; filters, demand cap, lookup cap, cache)

## Command

```console
cargo test -p astromech-market --test here
```

Exit status: 0

## Stdout

Bytes: 491
Truncated: no

```text

running 6 tests
test market_symbols_are_ardents ... ok
test lookups_are_capped_carried_first_then_the_goods_furthest_below_average ... ok
test carried_goods_need_a_market_with_demand_for_all_of_them ... ok
test the_lookups_filters_apply_to_the_nearby_markets ... ok
test the_hold_and_the_goods_for_sale_against_the_best_nearby_prices ... ok
test a_second_look_is_answered_from_the_cache ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 202
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running tests/here.rs (target/debug/deps/here-3f02193e50d153c0)
```

