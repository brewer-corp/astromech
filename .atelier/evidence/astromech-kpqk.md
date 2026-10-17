---
created_at: "2026-10-16T23:35:09.902143253+00:00"
id: "astromech-kpqk"
evidence_type: "test"
commit: "ba77ef85bf23f20da5764d5aa18a93113fe17139"
result: "pass"
captured_at: "2026-10-16T23:35:09.059549202+00:00"
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
title: "Trade routes from the recorded Spansh exchange and Ardent markets: single hop with profit per ton and estimated per hour, loop return leg (hold, credits, stock, data age), start station pick, no route and errors"
updated_at: "2026-10-16T23:35:09.902501302+00:00"
---

## Summary

Trade routes from the recorded Spansh exchange and Ardent markets: single hop with profit per ton and estimated per hour, loop return leg (hold, credits, stock, data age), start station pick, no route and errors

## Command

```console
cargo test -p astromech-market --test route
```

Exit status: 0

## Stdout

Bytes: 323
Truncated: no

```text

running 4 tests
test single_hop_from_a_recorded_job_and_its_polls ... ok
test no_route_and_errors ... ok
test without_a_station_starts_nearest_the_star_that_fits ... ok
test loop_brings_the_best_cargo_back_to_the_start ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 384
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.74s
     Running tests/route.rs (target/debug/deps/route-671f6657f42248a9)
```

