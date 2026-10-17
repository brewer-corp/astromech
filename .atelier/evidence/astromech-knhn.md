---
created_at: "2026-10-17T01:45:16.062564703+00:00"
id: "astromech-knhn"
evidence_type: "test"
commit: "4742186192442e88192347b67b399906d2adce92"
result: "pass"
captured_at: "2026-10-17T01:45:14.587830348+00:00"
command: "cargo test -p astromech-announce --test arrival --test replay -p astromech-core --test copy"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-nugx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-nugx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Arrival replay tests (crates/astromech-announce/tests/arrival.rs): a Boom/Famine/War/Civil War/Civil Unrest system whose state gives a material on the shopping list gets its state line; unwanted states, an empty list and a Federation Boom stay quiet; a visited, discovered system with nothing notable (StartJump with StarClass, FSDJump with security, AutoScan WasDiscovered true) gets nothing; an undiscovered arrival star gives 'Undiscovered system.'; pledged commander hears Undermining/Acquisition target, not reinforce; star class, security and allegiance never spoken; catch-up silent. 5 of 7 failed before the rule was registered. Copy guide covers every arrival line."
updated_at: "2026-10-17T01:45:16.063348525+00:00"
---

## Summary

Arrival replay tests (crates/astromech-announce/tests/arrival.rs): a Boom/Famine/War/Civil War/Civil Unrest system whose state gives a material on the shopping list gets its state line; unwanted states, an empty list and a Federation Boom stay quiet; a visited, discovered system with nothing notable (StartJump with StarClass, FSDJump with security, AutoScan WasDiscovered true) gets nothing; an undiscovered arrival star gives 'Undiscovered system.'; pledged commander hears Undermining/Acquisition target, not reinforce; star class, security and allegiance never spoken; catch-up silent. 5 of 7 failed before the rule was registered. Copy guide covers every arrival line.

## Command

```console
cargo test -p astromech-announce --test arrival --test replay -p astromech-core --test copy
```

Exit status: 0

## Stdout

Bytes: 1234
Truncated: no

```text

running 7 tests
test an_undiscovered_system_is_said_after_the_jump ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test bad_settings_are_reported_not_fatal ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test a_disabled_rule_never_fires ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok
test quiet_during_combat ... ok
test quiet_during_docking ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 536
Truncated: no

```text
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-announce)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-77a1/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.29s
     Running tests/arrival.rs (target/debug/deps/arrival-0f9ae8ccfdeb11df)
     Running tests/replay.rs (target/debug/deps/replay-268d4dfdd373d3ff)
     Running tests/copy.rs (target/debug/deps/copy-31003c62a4b6019e)
```

