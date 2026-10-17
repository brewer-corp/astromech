---
created_at: "2026-10-17T01:57:57.099475964+00:00"
id: "astromech-u4nf"
evidence_type: "test"
commit: "f9444f31d53e99506996f2cf6445cd9b8b9e9ff9"
result: "pass"
captured_at: "2026-10-17T01:57:54.231787976+00:00"
command: "bash -c 'cargo test -p astromech-announce --test route --test replay && cargo test -p astromech-core --lib plotted_route && cargo test -p astromech-core --test copy'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-2oxr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-2oxr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Route replay tests (crates/astromech-announce/tests/route.rs): a route fixture with a white dwarf and an 18-jump stretch a 32 t tank can't cross gives one line each (fuel gap, summary, white dwarf, neutron stars); a plain route gives only 'Route: 4 jumps, 50 ly.'; too little fuel for the first scoopable star from a non-scoopable start warns, a scoopable start does not; the same route again and a cleared route are quiet; no fuel line before a jump; catch-up silent. 5 of 6 failed with the rule unregistered. Pipeline test: the NavRoute.json read at startup and repeated reads don't fire, a new route fires once. Copy guide covers every line."
updated_at: "2026-10-17T01:57:57.099962531+00:00"
---

## Summary

Route replay tests (crates/astromech-announce/tests/route.rs): a route fixture with a white dwarf and an 18-jump stretch a 32 t tank can't cross gives one line each (fuel gap, summary, white dwarf, neutron stars); a plain route gives only 'Route: 4 jumps, 50 ly.'; too little fuel for the first scoopable star from a non-scoopable start warns, a scoopable start does not; the same route again and a cleared route are quiet; no fuel line before a jump; catch-up silent. 5 of 6 failed with the rule unregistered. Pipeline test: the NavRoute.json read at startup and repeated reads don't fire, a new route fires once. Copy guide covers every line.

## Command

```console
bash -c 'cargo test -p astromech-announce --test route --test replay && cargo test -p astromech-core --lib plotted_route && cargo test -p astromech-core --test copy'
```

Exit status: 0

## Stdout

Bytes: 1379
Truncated: no

```text

running 8 tests
test catch_up_leaves_docking_quiet_in_place ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test bad_settings_are_reported_not_fatal ... ok
test a_disabled_rule_never_fires ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test the_route_briefing_is_silent_during_the_catch_up ... ok
test too_little_fuel_for_the_first_scoopable_star_is_a_fuel_gap_too ... ok
test without_a_jump_to_go_by_there_is_no_fuel_line ... ok
test a_plain_route_gets_only_its_summary ... ok
test the_same_route_written_again_is_not_said_again_and_a_cleared_route_is_quiet ... ok
test a_white_dwarf_and_a_fuel_gap_give_one_hazard_line_each ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 0.00s


running 1 test
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 637
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running tests/replay.rs (target/debug/deps/replay-47133f362d1a518e)
     Running tests/route.rs (target/debug/deps/route-2367fded4adc653a)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-38eef3c8cf4fd98c)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.12s
     Running tests/copy.rs (target/debug/deps/copy-31003c62a4b6019e)
```

