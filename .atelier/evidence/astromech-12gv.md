---
created_at: "2026-10-17T19:03:09.414053161+00:00"
id: "astromech-12gv"
evidence_type: "test"
commit: "85638729cdc6806a560cda6e833a8831fa779c80"
result: "pass"
captured_at: "2026-10-17T19:02:30.422130468+00:00"
command: "cargo test -p astromech-announce --test prospector"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-u8ir"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-u8ir"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-announce --test prospector"
updated_at: "2026-10-17T19:03:09.414679460+00:00"
---

## Summary

cargo test -p astromech-announce --test prospector

## Command

```console
cargo test -p astromech-announce --test prospector
```

Exit status: 0

## Stdout

Bytes: 485
Truncated: no

```text

running 7 tests
test a_rock_below_threshold_gets_nothing ... ok
test a_rock_below_threshold_is_not_remembered ... ok
test threshold_changes_from_settings_apply_at_once ... ok
test a_rock_above_threshold_gets_one_short_line ... ok
test a_depleted_rock_gets_only_its_motherlode_line ... ok
test a_motherlode_always_gets_a_line ... ok
test the_same_rock_prospected_again_is_said_once ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 324
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-announce)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.72s
     Running tests/prospector.rs (target/debug/deps/prospector-861e528bdfe01087)
```

