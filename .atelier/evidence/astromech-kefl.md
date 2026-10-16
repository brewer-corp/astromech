---
created_at: "2026-10-16T20:48:53.602667481+00:00"
id: "astromech-kefl"
evidence_type: "test"
commit: "948ca35bd0b58e2775e810f85ac4b189ba85de9d"
result: "pass"
captured_at: "2026-10-16T20:48:50.671353277+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-galaxy --test powerplay && cargo test -p astromech-data && cd app && pnpm vitest run src/modules/powerplay'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-7ix7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-7ix7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded Spansh responses (captured live 16 Oct 2026 around Lembava): reinforcement, acquisition and undermining searches send the verified filter shapes and return the pledged power's systems in distance order (also when Spansh's order is reversed), with state, progress and acquisition share; answers that ignore the filter are dropped with a warning; age filter and cache; data-layer schema test for the Powerplay fields; app vitest for the Where to work panel"
updated_at: "2026-10-16T20:48:53.603122611+00:00"
---

## Summary

Recorded Spansh responses (captured live 16 Oct 2026 around Lembava): reinforcement, acquisition and undermining searches send the verified filter shapes and return the pledged power's systems in distance order (also when Spansh's order is reversed), with state, progress and acquisition share; answers that ignore the filter are dropped with a warning; age filter and cache; data-layer schema test for the Powerplay fields; app vitest for the Where to work panel

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-galaxy --test powerplay && cargo test -p astromech-data && cd app && pnpm vitest run src/modules/powerplay'
```

Exit status: 0

## Stdout

Bytes: 1879
Truncated: no

```text

running 6 tests
test answers_that_ignore_the_filter_are_dropped_with_a_warning ... ok
test acquisition_systems_carry_the_powers_progress ... ok
test reinforcement_systems_are_the_pledged_powers_nearest_first ... ok
test hits_are_in_distance_order_whatever_order_spansh_answers_in ... ok
test undermining_systems_belong_to_other_powers ... ok
test the_age_filter_applies_and_the_answer_is_cached ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test spansh::tests::search_body_uses_verified_shapes ... ok
test transport::tests::segments_and_cache_keys ... ok
test limit::tests::rate_limit_spaces_requests_per_service ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 12 tests
test http_errors_are_plain_words ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test uncached_calls_always_ask ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test spansh_route_job_round_trip ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test spansh_system_names_autocomplete ... ok
test no_network_gives_a_plain_words_error ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok
test spansh_searches_parse_recorded_answers ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-7ni2/app


 Test Files  2 passed (2)
      Tests  6 passed (6)
   Start at  16:48:52
   Duration  1.01s (environment 55%, tests 17%, setup 14%, transform 11%, import 2%, worker 1%)
```

## Stderr

Bytes: 539
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.84s
     Running tests/powerplay.rs (target/debug/deps/powerplay-b5fb3d56ce42257a)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/astromech_data-70140388ab27b685)
     Running tests/recorded.rs (target/debug/deps/recorded-5f1e7ec202a06bdb)
   Doc-tests astromech_data
```

