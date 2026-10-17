---
created_at: "2026-10-17T19:20:53.300530518+00:00"
id: "astromech-h2lr"
evidence_type: "test"
commit: "274db91e27cb227724291c92e04e2554d8681889"
result: "pass"
captured_at: "2026-10-17T19:20:16.510238172+00:00"
command: "cargo test -p astromech-core --test voice_tester a_learned_phrase"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-znvi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-znvi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --test voice_tester a_learned_phrase"
updated_at: "2026-10-17T19:20:53.300804138+00:00"
---

## Summary

cargo test -p astromech-core --test voice_tester a_learned_phrase

## Command

```console
cargo test -p astromech-core --test voice_tester a_learned_phrase
```

Exit status: 0

## Stdout

Bytes: 158
Truncated: no

```text

running 1 test
test a_learned_phrase_can_be_forgotten ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.36s
```

## Stderr

Bytes: 320
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.17s
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-ead47f3575d86951)
```

