---
created_at: "2026-10-16T18:54:26.602567461+00:00"
id: "astromech-sim8"
evidence_type: "test"
commit: "d558046708ca02368688bd0c6984f80e1ec846a8"
result: "pass"
captured_at: "2026-10-16T18:54:26.322246287+00:00"
command: "cargo test -p astromech-ship"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-ojwr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ojwr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-ship: 5 reference builds (Krait Mk II + FdL combat, Anaconda + Krait Phantom exploration, Python mining; charge-enhanced and weapon-focused PDs, Mass Manager and Deep Charge FSDs, thermal-resistant boosters, Guardian FSD booster) match Coriolis to its displayed rounding on all 35 summary stats; expectations from Coriolis's own code (scripts/coriolis-ref, coriolis@68c042c, coriolis-data@0db9234); full-precision values agree to ~1e-16"
updated_at: "2026-10-16T18:54:26.603060191+00:00"
---

## Summary

astromech-ship: 5 reference builds (Krait Mk II + FdL combat, Anaconda + Krait Phantom exploration, Python mining; charge-enhanced and weapon-focused PDs, Mass Manager and Deep Charge FSDs, thermal-resistant boosters, Guardian FSD booster) match Coriolis to its displayed rounding on all 35 summary stats; expectations from Coriolis's own code (scripts/coriolis-ref, coriolis@68c042c, coriolis-data@0db9234); full-precision values agree to ~1e-16

## Command

```console
cargo test -p astromech-ship
```

Exit status: 0

## Stdout

Bytes: 718
Truncated: no

```text

running 4 tests
test tests::capacitor_recharge_follows_the_pip_curve ... ok
test tests::unknown_ship_is_an_error ... ok
test tests::unpowered_drive_has_no_range ... ok
test tests::unknown_modules_and_blueprints_are_reported_not_counted ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 5 tests
test anaconda_explorer ... ok
test python_miner ... ok
test fer_de_lance_combat ... ok
test krait_phantom_explorer ... ok
test krait_mkii_combat ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 264
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-8eaa524fdea4b627)
     Running tests/coriolis.rs (target/debug/deps/coriolis-4c757b90eb068af0)
   Doc-tests astromech_ship
```

