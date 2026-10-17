---
created_at: "2026-10-17T01:53:57.273713302+00:00"
id: "astromech-fhue"
evidence_type: "test"
commit: "ea0c841dfc5f8249afad834b9bafb55019b39a91"
result: "pass"
captured_at: "2026-10-17T01:53:53.290289346+00:00"
command: "cargo test -p astromech-announce --test fuel --test replay -p astromech-journal -p astromech-core --test copy"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6jzn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6jzn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fuel replay tests (crates/astromech-announce/tests/fuel.rs) over NavRoute.json fixtures: a route with a reachable scoopable star gives 'Next scoopable star in N jumps.' / 'Next star is scoopable.' on each non-scoopable arrival, nothing at a scoopable star or at the destination; a route without enough fuel (8 t needed, 5 t left) warns once across three jumps; enough fuel gives no warning; at a scoopable star short of the next one 'Scoop here. Next scoopable star out of reach.', with the FSD power law tested (linear scaling would not warn); off the route, no route and catch-up stay quiet. 5 of 7 failed before the rules existed. Journal parse test covers hops and FuelUsed. Copy guide covers every line."
updated_at: "2026-10-17T01:53:57.274124810+00:00"
---

## Summary

Fuel replay tests (crates/astromech-announce/tests/fuel.rs) over NavRoute.json fixtures: a route with a reachable scoopable star gives 'Next scoopable star in N jumps.' / 'Next star is scoopable.' on each non-scoopable arrival, nothing at a scoopable star or at the destination; a route without enough fuel (8 t needed, 5 t left) warns once across three jumps; enough fuel gives no warning; at a scoopable star short of the next one 'Scoop here. Next scoopable star out of reach.', with the FSD power law tested (linear scaling would not warn); off the route, no route and catch-up stay quiet. 5 of 7 failed before the rules existed. Journal parse test covers hops and FuelUsed. Copy guide covers every line.

## Command

```console
cargo test -p astromech-announce --test fuel --test replay -p astromech-journal -p astromech-core --test copy
```

Exit status: 0

## Stdout

Bytes: 1199
Truncated: no

```text

running 7 tests
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test the_next_scoopable_star_is_said_on_arrival_at_one_that_is_not ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test enough_fuel_gives_no_warning ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test a_disabled_rule_never_fires ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 1792
Truncated: no

```text
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-journal)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-materials)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-powerplay)
   Compiling astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-history)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-gamestate)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-galaxy)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-engineering)
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-announce)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-market)
   Compiling astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-station)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.84s
     Running tests/fuel.rs (target/debug/deps/fuel-77d59fb215bff780)
     Running tests/replay.rs (target/debug/deps/replay-268d4dfdd373d3ff)
     Running tests/copy.rs (target/debug/deps/copy-31003c62a4b6019e)
```

