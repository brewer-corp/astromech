---
created_at: "2026-10-16T19:03:32.352575565+00:00"
id: "astromech-s0j3"
evidence_type: "test"
commit: "f847b9f8d2e23cf61d7e96a645ccf9200a08c9fa"
result: "pass"
captured_at: "2026-10-16T19:03:26.956094720+00:00"
command: "cargo test -p astromech-data -p astromech-galaxy"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-h74l"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-h74l"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes: astromech-data recorded-response tests (Done-when: cached_answer_is_served_without_a_request_until_ttl_ends, schema_mismatch_is_an_error_naming_the_service, no_network_gives_a_plain_words_error) and galaxy's nearby_fetches_through_the_data_client_then_reuses_the_cache"
updated_at: "2026-10-16T19:03:32.369215171+00:00"
---

## Summary

After review fixes: astromech-data recorded-response tests (Done-when: cached_answer_is_served_without_a_request_until_ttl_ends, schema_mismatch_is_an_error_naming_the_service, no_network_gives_a_plain_words_error) and galaxy's nearby_fetches_through_the_data_client_then_reuses_the_cache

## Command

```console
cargo test -p astromech-data -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 1689
Truncated: no

```text

running 3 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok
test transport::tests::segments_and_cache_keys ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
test uncached_calls_always_ask ... ok
test spansh_route_job_round_trip ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test spansh_searches_parse_recorded_answers ... ok
test http_errors_are_plain_words ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test no_network_gives_a_plain_words_error ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 7 tests
test remote::tests::parse_and_match_fixtures ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test tests::here_from_journals ... ok
test tests::farming_notes_name_known_lookups ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test store::tests::seed_insert_cover_cache ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 661
Truncated: no

```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.53s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-b357499db75b714c)
     Running tests/recorded.rs (target/debug/deps/recorded-5fe148157514a7c9)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-7101e172e17599b6)
   Doc-tests astromech_data
   Doc-tests astromech_galaxy
```

