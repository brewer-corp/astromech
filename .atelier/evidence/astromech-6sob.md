---
created_at: "2026-10-16T19:13:08.339700814+00:00"
id: "astromech-6sob"
evidence_type: "test"
commit: "bf12a092ad8a368c3896df9b056e08b5d6b83c5e"
result: "pass"
captured_at: "2026-10-16T19:13:07.829732512+00:00"
command: "cargo test -p astromech-data -p astromech-galaxy"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-pxy2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-pxy2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-data autocomplete (recorded Spansh answer) and galaxy typed-origin tests pass"
updated_at: "2026-10-16T19:13:08.339904214+00:00"
---

## Summary

astromech-data autocomplete (recorded Spansh answer) and galaxy typed-origin tests pass

## Command

```console
cargo test -p astromech-data -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 1801
Truncated: no

```text

running 3 tests
test transport::tests::segments_and_cache_keys ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok
test limit::tests::rate_limit_spaces_requests_per_service ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 12 tests
test spansh_route_job_round_trip ... ok
test uncached_calls_always_ask ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test spansh_system_names_autocomplete ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test http_errors_are_plain_words ... ok
test no_network_gives_a_plain_words_error ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test spansh_searches_parse_recorded_answers ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test tests::farming_notes_name_known_lookups ... ok
test tests::here_from_journals ... ok
test remote::tests::parse_and_match_fixtures ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test store::tests::seed_insert_cover_cache ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 382
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-b357499db75b714c)
     Running tests/recorded.rs (target/debug/deps/recorded-5fe148157514a7c9)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-7101e172e17599b6)
   Doc-tests astromech_data
   Doc-tests astromech_galaxy
```

