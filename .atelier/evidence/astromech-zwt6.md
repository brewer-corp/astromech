---
created_at: "2026-10-22T23:05:31.178720484+00:00"
id: "astromech-zwt6"
evidence_type: "test"
commit: "eb5b71b2fdbffdd1dadc903364da4756c4b78d2d"
result: "pass"
captured_at: "2026-10-22T23:05:12.858632430+00:00"
command: "env NEUTRON_BIN=/tmp/n6bjl.aLAx/neutron.bin CARGO_BUILD_JOBS=4 cargo test --release -p astromech-galaxy --test neutron_plot -- --ignored --nocapture"
exit_status: "0"
agent_identity: "claude-implementer-6bjl"
target:
  kind: "issue"
  id: "astromech-6bjl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6bjl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "On the real 22 Oct 2026 neutron star file (built with scripts/gen-neutron.py, not committed): local routes have fewer jumps than Spansh's recorded routes in the model (Colonia 50/x4 139 vs 160, Beagle 60/x4 383 vs 438, Beagle 80/x6 161 vs 181) and each timed route searches in under 2 s"
updated_at: "2026-10-22T23:05:31.179240853+00:00"
---

## Summary

On the real 22 Oct 2026 neutron star file (built with scripts/gen-neutron.py, not committed): local routes have fewer jumps than Spansh's recorded routes in the model (Colonia 50/x4 139 vs 160, Beagle 60/x4 383 vs 438, Beagle 80/x6 161 vs 181) and each timed route searches in under 2 s

## Command

```console
env NEUTRON_BIN=/tmp/n6bjl.aLAx/neutron.bin CARGO_BUILD_JOBS=4 cargo test --release -p astromech-galaxy --test neutron_plot -- --ignored --nocapture
```

Exit status: 0

## Stdout

Bytes: 1021
Truncated: no

```text

running 1 test
loaded 5045154 neutron stars in 202.188184ms, peak 153 MB
spansh-neutron-sol-colonia-50-4.json: local 139 jumps, 119 stops; Spansh 166 jumps (160 in the model), 128 stops
spansh-neutron-sol-beagle-60-4.json: local 383 jumps, 279 stops; Spansh 467 jumps (438 in the model), 298 stops
spansh-neutron-sol-beagle-80-6.json: local 161 jumps, 146 stops; Spansh 190 jumps (181 in the model), 150 stops
Sol to Colonia, R = 60, k = 4: 114 jumps, 101 stops; search 514.377073ms, names 1.246723ms; peak 140 MB (80 MB before)
Sol to Colonia, R = 80, k = 6: 53 jumps, 51 stops; search 708.117358ms, names 776.616µs; peak 142 MB (88 MB before)
Sol to Beagle Point, R = 60, k = 4: 383 jumps, 279 stops; search 496.366349ms, names 1.126679ms; peak 139 MB (94 MB before)
Sol to Beagle Point, R = 80, k = 6: 161 jumps, 146 stops; search 1.073241297s, names 811.331µs; peak 141 MB (84 MB before)
test on_the_real_file ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 4.94s
```

## Stderr

Bytes: 273
Truncated: no

```text
   Compiling astromech-galaxy v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-1842/crates/astromech-galaxy)
    Finished `release` profile [optimized] target(s) in 13.19s
     Running tests/neutron_plot.rs (target/release/deps/neutron_plot-31693c742e571d16)
```

