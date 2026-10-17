---
created_at: "2026-10-17T19:13:36.519965190+00:00"
id: "astromech-e2ic"
evidence_type: "test"
commit: "fca4abe01fb5dd1bcef5cb1b8d470296a48529ff"
result: "pass"
captured_at: "2026-10-17T19:13:27.821678507+00:00"
command: "cargo test -p astromech-core --lib config::"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-thym"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-thym"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib config::"
updated_at: "2026-10-17T19:13:36.520247506+00:00"
---

## Summary

cargo test -p astromech-core --lib config::

## Command

```console
cargo test -p astromech-core --lib config::
```

Exit status: 0

## Stdout

Bytes: 364
Truncated: no

```text

running 4 tests
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::one_bad_value_costs_only_that_value ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.00s
```

## Stderr

Bytes: 322
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-config/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.52s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
```

