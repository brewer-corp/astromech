---
created_at: "2026-10-16T18:56:09.432829019+00:00"
id: "astromech-d17c"
evidence_type: "test"
commit: "a2be550e3cb1d49dea8fc88e409a5e6a5011acf0"
result: "pass"
captured_at: "2026-10-16T18:56:08.160327007+00:00"
command: "cargo test -p astromech-data"
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
title: "astromech-data on recorded responses: cached_answer_is_served_without_a_request_until_ttl_ends, schema_mismatch_is_an_error_naming_the_service, no_network_gives_a_plain_words_error, plus endpoint parsing, HTTP errors, rate limiter"
updated_at: "2026-10-16T18:56:09.433108801+00:00"
---

## Summary

astromech-data on recorded responses: cached_answer_is_served_without_a_request_until_ttl_ends, schema_mismatch_is_an_error_naming_the_service, no_network_gives_a_plain_words_error, plus endpoint parsing, HTTP errors, rate limiter

## Command

```console
cargo test -p astromech-data
```

Exit status: 0

## Stdout

Bytes: 1084
Truncated: no

```text

running 3 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test transport::tests::segments_and_cache_keys ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
test uncached_calls_always_ask ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test spansh_route_job_round_trip ... ok
test http_errors_are_plain_words ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test no_network_gives_a_plain_words_error ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test spansh_searches_parse_recorded_answers ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 383
Truncated: no

```text
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-data)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-70140388ab27b685)
     Running tests/recorded.rs (target/debug/deps/recorded-5f1e7ec202a06bdb)
   Doc-tests astromech_data
```

