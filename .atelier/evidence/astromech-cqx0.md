---
created_at: "2026-10-17T02:46:09.928087621+00:00"
id: "astromech-cqx0"
evidence_type: "test"
commit: "09fe9582eb59b2b43dd660eb6787cc5b134da783"
result: "pass"
captured_at: "2026-10-17T02:46:07.378097989+00:00"
command: "cargo test -p astromech-galaxy --test colonies"
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
title: "Done-when: recorded Spansh answers (Greae Phoea EM-K d9-675, 30 ly, 17 Oct 2026) give exactly the 17 unclaimed, unpopulated systems within 15 ly of a populated one; claimed, out-of-range and stale-distance cases covered; mutations (ignore claims, 20 ly range) fail it"
updated_at: "2026-10-17T02:46:09.929071516+00:00"
---

## Summary

Done-when: recorded Spansh answers (Greae Phoea EM-K d9-675, 30 ly, 17 Oct 2026) give exactly the 17 unclaimed, unpopulated systems within 15 ly of a populated one; claimed, out-of-range and stale-distance cases covered; mutations (ignore claims, 20 ly range) fail it

## Command

```console
cargo test -p astromech-galaxy --test colonies
```

Exit status: 0

## Stdout

Bytes: 576
Truncated: no

```text

running 7 tests
test an_area_where_everything_is_claimed_gives_nothing_without_a_warning ... ok
test an_ignored_population_filter_gives_nothing_and_a_warning ... ok
test the_age_filter_and_the_cache_apply ... ok
test spansh_distances_alone_skip_the_populated_search ... ok
test a_recorded_answer_gives_only_unclaimed_systems_within_15_ly_of_a_populated_one ... ok
test a_truncated_populated_list_counts_what_it_couldnt_check ... ok
test bodies_of_interest_narrow_the_list ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

## Stderr

Bytes: 210
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running tests/colonies.rs (target/debug/deps/colonies-42f51fbd7ef376d8)
```

