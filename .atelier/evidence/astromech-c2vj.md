---
created_at: "2026-10-16T20:53:57.661394212+00:00"
id: "astromech-c2vj"
evidence_type: "test"
commit: "53fbc221b9282183e33c953f4a0409931f30fb62"
result: "pass"
captured_at: "2026-10-16T20:53:51.316686717+00:00"
command: "cargo test -p astromech-galaxy --locked stations"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-tw2y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tw2y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded-response tests: every service, trader/broker type-only, filters sent and re-checked, Galaxy::stations"
updated_at: "2026-10-16T20:53:57.661807924+00:00"
---

## Summary

Recorded-response tests: every service, trader/broker type-only, filters sent and re-checked, Galaxy::stations

## Command

```console
cargo test -p astromech-galaxy --locked stations
```

Exit status: 0

## Stdout

Bytes: 613
Truncated: no

```text

running 7 tests
test stations::tests::service_ids_round_trip ... ok
test stations::tests::an_ignored_filter_gives_a_warning_not_wrong_stations ... ok
test stations::tests::pad_comes_from_pad_counts ... ok
test tests::stations_lists_the_nearest_with_jumps_from_the_typed_origin ... ok
test stations::tests::filters_go_to_spansh_and_are_rechecked ... ok
test stations::tests::traders_and_brokers_are_only_the_asked_for_type ... ok
test stations::tests::every_service_finds_stations_that_offer_it_nearest_first ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.05s
```

## Stderr

Bytes: 284
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.06s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-d5a668c68e3f5656)
```

