---
created_at: "2026-10-16T19:18:40.729877965+00:00"
id: "astromech-y59t"
evidence_type: "test"
commit: "1e28693cf90399521cb61803562b30d2c5ce6555"
result: "pass"
captured_at: "2026-10-16T19:18:35.584692705+00:00"
command: "cargo test -p astromech-galaxy"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-1imw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-1imw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Filter persistence in Rust: filters saved to an astromech.sqlite file survive closing and reopening the store, for that lookup only; pad follows a ship change (krait M -> anaconda L) unless set by the commander (S, or Any), per lookup; keeps() applies each filter; Galaxy::nearby applies data age."
updated_at: "2026-10-16T19:18:40.730279243+00:00"
---

## Summary

Filter persistence in Rust: filters saved to an astromech.sqlite file survive closing and reopening the store, for that lookup only; pad follows a ship change (krait M -> anaconda L) unless set by the commander (S, or Any), per lookup; keeps() applies each filter; Galaxy::nearby applies data age.

## Command

```console
cargo test -p astromech-galaxy
```

Exit status: 0

## Stdout

Bytes: 881
Truncated: no

```text

running 11 tests
test filters::tests::keeps_applies_each_filter ... ok
test tests::farming_notes_name_known_lookups ... ok
test remote::tests::parse_and_match_fixtures ... ok
test map::tests::exact_on_map_estimate_off_map ... ok
test tests::here_from_journals ... ok
test tests::typed_origin_is_found_in_the_map_then_on_spansh ... ok
test filters::tests::pad_follows_the_ship_unless_the_commander_set_it ... ok
test tests::nearby_ranks_cached_hits_by_jumps ... ok
test tests::nearby_fetches_through_the_data_client_then_reuses_the_cache ... ok
test filters::tests::filters_survive_a_restart_for_their_lookup_only ... ok
test store::tests::seed_insert_cover_cache ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 314
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.66s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-d5a668c68e3f5656)
   Doc-tests astromech_galaxy
```

