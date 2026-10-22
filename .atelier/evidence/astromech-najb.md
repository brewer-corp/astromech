---
created_at: "2026-10-22T20:48:38.597324247+00:00"
id: "astromech-najb"
evidence_type: "test"
commit: "141d4e4de123567f226682b21e1df86fa00cfd1d"
result: "pass"
captured_at: "2026-10-22T20:47:43.930038926+00:00"
command: "env CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test plot"
exit_status: "0"
agent_identity: "claude-implementer-vlsz"
target:
  kind: "issue"
  id: "astromech-vlsz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-vlsz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech_galaxy::plot sends the ship Fsd's supercharge_multiplier: 4 for the Asp loadout on the exact plotter, 6 on the neutron plotter for a Mk II drive"
updated_at: "2026-10-22T20:48:38.597649113+00:00"
---

## Summary

astromech_galaxy::plot sends the ship Fsd's supercharge_multiplier: 4 for the Asp loadout on the exact plotter, 6 on the neutron plotter for a Mk II drive

## Command

```console
env CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test plot
```

Exit status: 0

## Stdout

Bytes: 326
Truncated: no

```text

running 4 tests
test the_current_ship_gives_the_range_and_drive ... ok
test a_neutron_route_stops_at_the_neutron_stars ... ok
test an_exact_route_has_every_jump_and_its_refuel_stops ... ok
test plot_failures_are_sentences ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 202
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running tests/plot.rs (target/debug/deps/plot-64b3010cdd832c24)
```

