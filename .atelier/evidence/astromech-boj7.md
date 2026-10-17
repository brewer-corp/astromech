---
created_at: "2026-10-17T17:07:05.544681593+00:00"
id: "astromech-boj7"
evidence_type: "test"
commit: "2fa91fbabf520e0b0128ed2e6e9649e5c6e60f34"
result: "pass"
captured_at: "2026-10-17T17:07:05.267039890+00:00"
command: "cargo test -p astromech-ship"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-tgar"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tgar"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-ship"
updated_at: "2026-10-17T17:07:05.545025975+00:00"
---

## Summary

cargo test -p astromech-ship

## Command

```console
cargo test -p astromech-ship
```

Exit status: 0

## Stdout

Bytes: 1517
Truncated: no

```text

running 10 tests
test tests::capacitor_recharge_follows_the_pip_curve ... ok
test tests::unknown_ship_is_an_error ... ok
test module::tests::modifiers_set_the_stats_and_unknown_labels_are_reported ... ok
test tests::unpowered_drive_has_no_range ... ok
test module::tests::rail_gun_sustained_counts_its_charge ... ok
test module::tests::unknown_grade_is_reported ... ok
test module::tests::grade_5_at_roll_0_is_the_grade_4_maximum ... ok
test tests::unknown_hull_is_incomplete_not_an_error ... ok
test tests::unknown_modules_and_blueprints_are_reported_not_counted ... ok
test module::tests::sustained_never_exceeds_burst_for_any_catalogue_module ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 15 tests
test fer_de_lance_modifiers ... ok
test krait_phantom_explorer ... ok
test fer_de_lance_combat ... ok
test anaconda_explorer ... ok
test fer_de_lance_sustained_counts_charge ... ok
test krait_mkii_combat ... ok
test card_headline_for_the_krait_mkii_combat_build ... ok
test anaconda_legacy ... ok
test plan_legacy_fsd_long_range ... ok
test plan_pd_cluster_capacitors ... ok
test plan_pd_super_conduits ... ok
test plan_fsd_fast_boot ... ok
test python_miner ... ok
test plan_thermal_boosters_4 ... ok
test plan_thermal_boosters_3 ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 264
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-03f630623ad11695)
     Running tests/coriolis.rs (target/debug/deps/coriolis-152f473152671da0)
   Doc-tests astromech_ship
```

