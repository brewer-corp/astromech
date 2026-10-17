---
created_at: "2026-10-17T02:46:13.014931939+00:00"
id: "astromech-pgga"
evidence_type: "test"
commit: "09fe9582eb59b2b43dd660eb6787cc5b134da783"
result: "pass"
captured_at: "2026-10-17T02:46:10.076244751+00:00"
command: "cargo test -p astromech-data"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-20dy"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-20dy"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Spansh client: population and system_id64 filter shapes, colonisation fields parse"
updated_at: "2026-10-17T02:46:13.015353906+00:00"
---

## Summary

Spansh client: population and system_id64 filter shapes, colonisation fields parse

## Command

```console
cargo test -p astromech-data
```

Exit status: 0

## Stdout

Bytes: 1242
Truncated: no

```text

running 3 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test transport::tests::segments_and_cache_keys ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 14 tests
test edsm_endpoints_parse_recorded_answers ... ok
test spansh_searches_parse_recorded_answers ... ok
test http_errors_are_plain_words ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test spansh_trade_route_polls_until_the_job_is_done ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test no_network_gives_a_plain_words_error ... ok
test spansh_route_job_round_trip ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test spansh_system_names_autocomplete ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok
test uncached_calls_always_ask ... ok
test spansh_trade_route_errors_are_plain_words ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 380
Truncated: no

```text
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-20dy/crates/astromech-data)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.34s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-f8cf89e6f329a93d)
     Running tests/recorded.rs (target/debug/deps/recorded-3dcf0cb75e4928d2)
   Doc-tests astromech_data
```

