---
created_at: "2026-10-16T19:21:58.051897972+00:00"
id: "astromech-8di7"
evidence_type: "test"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:21:57.423210325+00:00"
command: "cargo test -p astromech-ship --test coriolis"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-ship --test coriolis"
updated_at: "2026-10-16T19:21:58.052295895+00:00"
---

## Summary

cargo test -p astromech-ship --test coriolis

## Command

```console
cargo test -p astromech-ship --test coriolis
```

Exit status: 0

## Stdout

Bytes: 444
Truncated: no

```text

running 10 tests
test fer_de_lance_combat ... ok
test anaconda_explorer ... ok
test krait_phantom_explorer ... ok
test krait_mkii_combat ... ok
test plan_thermal_boosters_4 ... ok
test plan_thermal_boosters_3 ... ok
test python_miner ... ok
test plan_fsd_fast_boot ... ok
test plan_pd_super_conduits ... ok
test plan_pd_cluster_capacitors ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 269
Truncated: no

```text
   Compiling astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-ship)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s
     Running tests/coriolis.rs (target/debug/deps/coriolis-4c757b90eb068af0)
```

