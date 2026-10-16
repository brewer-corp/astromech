---
created_at: "2026-10-16T19:35:26.490993206+00:00"
id: "astromech-kjv0"
evidence_type: "test"
commit: "27e5828139abb95e372a951c42fe8ea13d905cc0"
result: "pass"
captured_at: "2026-10-16T19:35:26.156463317+00:00"
command: "cargo test -p astromech-ship"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-9243"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-9243"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-ship"
updated_at: "2026-10-16T19:35:26.491778721+00:00"
---

## Summary

cargo test -p astromech-ship

## Command

```console
cargo test -p astromech-ship
```

Exit status: 0

## Stdout

Bytes: 1135
Truncated: no

```text

running 7 tests
test tests::capacitor_recharge_follows_the_pip_curve ... ok
test tests::unknown_ship_is_an_error ... ok
test tests::unpowered_drive_has_no_range ... ok
test module::tests::unknown_grade_is_reported ... ok
test module::tests::grade_5_at_roll_0_is_the_grade_4_maximum ... ok
test tests::unknown_hull_is_incomplete_not_an_error ... ok
test tests::unknown_modules_and_blueprints_are_reported_not_counted ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 11 tests
test anaconda_explorer ... ok
test krait_phantom_explorer ... ok
test python_miner ... ok
test card_headline_for_the_krait_mkii_combat_build ... ok
test fer_de_lance_combat ... ok
test plan_pd_cluster_capacitors ... ok
test plan_pd_super_conduits ... ok
test krait_mkii_combat ... ok
test plan_thermal_boosters_4 ... ok
test plan_thermal_boosters_3 ... ok
test plan_fsd_fast_boot ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 264
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-8eaa524fdea4b627)
     Running tests/coriolis.rs (target/debug/deps/coriolis-4c757b90eb068af0)
   Doc-tests astromech_ship
```

