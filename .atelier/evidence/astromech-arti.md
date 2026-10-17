---
created_at: "2026-10-17T01:49:28.853948815+00:00"
id: "astromech-arti"
evidence_type: "test"
commit: "92def2ad6c5f4725d6bdd9de5a7c4aaf9dcfb3a0"
result: "pass"
captured_at: "2026-10-17T01:49:25.916295584+00:00"
command: "cargo test -p astromech-announce --test docking --test replay -p astromech-core --test copy"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-lg9r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-lg9r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Docking replay tests (crates/astromech-announce/tests/docking.rs): Coriolis pads 1-45 give the clock position and depth from the layout table (17 pads checked across all three groups); Orbis and Ocellus share it; outposts, carriers, surface ports, pad 46 and an untyped station give nothing; each DockingDenied reason (10 known plus an unknown) gives its line; catch-up silent. 3 of 4 failed before the rule existed. Copy guide covers every pad and denial line."
updated_at: "2026-10-17T01:49:28.854383486+00:00"
---

## Summary

Docking replay tests (crates/astromech-announce/tests/docking.rs): Coriolis pads 1-45 give the clock position and depth from the layout table (17 pads checked across all three groups); Orbis and Ocellus share it; outposts, carriers, surface ports, pad 46 and an untyped station give nothing; each DockingDenied reason (10 known plus an unknown) gives its line; catch-up silent. 3 of 4 failed before the rule existed. Copy guide covers every pad and denial line.

## Command

```console
cargo test -p astromech-announce --test docking --test replay -p astromech-core --test copy
```

Exit status: 0

## Stdout

Bytes: 992
Truncated: no

```text

running 4 tests
test docking_callouts_are_silent_during_the_catch_up ... ok
test a_coriolis_pad_number_gives_its_position ... ok
test each_denial_reason_gets_its_line ... ok
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 8 tests
test a_rule_fires_once_per_event_within_its_cooldown ... ok
test bad_settings_are_reported_not_fatal ... ok
test nothing_is_spoken_during_the_journal_catch_up ... ok
test a_disabled_rule_never_fires ... ok
test catch_up_leaves_docking_quiet_in_place ... ok
test quiet_during_docking ... ok
test quiet_during_combat ... ok
test every_builtin_rule_is_silent_during_the_catch_up ... ok

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.79s
     Running tests/docking.rs (target/debug/deps/docking-f6328dd26da4f083)
     Running tests/replay.rs (target/debug/deps/replay-268d4dfdd373d3ff)
     Running tests/copy.rs (target/debug/deps/copy-31003c62a4b6019e)
```

