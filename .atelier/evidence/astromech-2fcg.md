---
created_at: "2026-10-16T21:04:58.594297616+00:00"
id: "astromech-2fcg"
evidence_type: "test"
commit: "710312cc432f4bb877c1a0127872134b0ee51152"
result: "pass"
captured_at: "2026-10-16T21:04:55.974223932+00:00"
command: "cargo test -p astromech-galaxy --locked -- stations outfitting"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-puas"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-puas"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded Spansh shipyard responses: nearest-first with price and shipyard age; a station without a large pad dropped for a Type-9 even with 'any pad'"
updated_at: "2026-10-16T21:04:58.594785718+00:00"
---

## Summary

Recorded Spansh shipyard responses: nearest-first with price and shipyard age; a station without a large pad dropped for a Type-9 even with 'any pad'

## Command

```console
cargo test -p astromech-galaxy --locked -- stations outfitting
```

Exit status: 0

## Stdout

Bytes: 1286
Truncated: no

```text

running 15 tests
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test outfitting::tests::mount_comes_from_the_symbol ... ok
test stations::tests::service_ids_round_trip ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test stations::tests::ships_too_big_for_a_stations_pads_are_left_out ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test stations::tests::shipyards_are_nearest_first_with_price ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 314
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-d5a668c68e3f5656)
   Doc-tests astromech_galaxy
```

