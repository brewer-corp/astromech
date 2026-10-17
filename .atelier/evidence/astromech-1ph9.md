---
created_at: "2026-10-16T23:43:54.496624002+00:00"
id: "astromech-1ph9"
evidence_type: "test"
commit: "1b6f53d79375bd1d307f01c09dc061ce9b5faecb"
result: "pass"
captured_at: "2026-10-16T23:43:46.009545304+00:00"
command: "cargo test -p astromech-market --test route"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mljg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mljg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes: route tests incl. pad warning (destination pad S for an M ship) and single-hop per hour equal to a loop flying back empty"
updated_at: "2026-10-16T23:43:54.529857860+00:00"
---

## Summary

After review fixes: route tests incl. pad warning (destination pad S for an M ship) and single-hop per hour equal to a loop flying back empty

## Command

```console
cargo test -p astromech-market --test route
```

Exit status: 0

## Stdout

Bytes: 454
Truncated: no

```text

running 6 tests
test single_hop_from_a_recorded_job_and_its_polls ... ok
test no_route_and_errors ... ok
test a_destination_too_small_for_the_ship_is_flagged ... ok
test without_a_station_starts_nearest_the_star_that_fits ... ok
test single_hop_per_hour_compares_with_a_loop_flying_back_empty ... ok
test loop_brings_the_best_cargo_back_to_the_start ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Stderr

Bytes: 324
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running tests/route.rs (target/debug/deps/route-671f6657f42248a9)
```

