---
created_at: "2026-10-16T21:00:56.972115006+00:00"
id: "astromech-ekq4"
evidence_type: "test"
commit: "b12c97bf37ab92242c11c46fbd50f91975796641"
result: "pass"
captured_at: "2026-10-16T21:00:55.854077130+00:00"
command: "cargo test -p astromech-galaxy --locked -- stations outfitting"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-4c6q"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-4c6q"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded Spansh module responses: nearest-first with price, pad L drops outposts, mount checked per station, outfitting age; catalogue"
updated_at: "2026-10-16T21:00:56.972333985+00:00"
---

## Summary

Recorded Spansh module responses: nearest-first with price, pad L drops outposts, mount checked per station, outfitting age; catalogue

## Command

```console
cargo test -p astromech-galaxy --locked -- stations outfitting
```

Exit status: 0

## Stdout

Bytes: 1142
Truncated: no

```text

running 13 tests
test outfitting::tests::mount_comes_from_the_symbol ... ok
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok
test stations::tests::service_ids_round_trip ... ok
test outfitting::tests::catalogue_has_modules_and_ships ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test stations::tests::modules_are_listed_nearest_first_with_price_and_honour_the_pad ... ok
test stations::tests::the_mount_is_checked_on_each_station ... ok
test stations::tests::module_age_is_the_outfitting_age ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 314
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-d5a668c68e3f5656)
   Doc-tests astromech_galaxy
```

