---
created_at: "2026-10-16T19:37:37.096014726+00:00"
id: "astromech-bhoo"
evidence_type: "test"
commit: "27015facc5498f1060161a890fd0616a1bdbc873"
result: "pass"
captured_at: "2026-10-16T19:37:33.744982601+00:00"
command: "cargo test -p astromech-voice speakable"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-w8ai"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-w8ai"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Table test maps each example (procedural names, catalogue prefixes, ores, percentages, large numbers, units) to expected TTS input; mutating the mass-code dash fails it"
updated_at: "2026-10-16T19:37:37.096380588+00:00"
---

## Summary

Table test maps each example (procedural names, catalogue prefixes, ores, percentages, large numbers, units) to expected TTS input; mutating the mass-code dash fails it

## Command

```console
cargo test -p astromech-voice speakable
```

Exit status: 0

## Stdout

Bytes: 163
Truncated: no

```text

running 1 test
test speakable::tests::reading_rules_table ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s
```

## Stderr

Bytes: 287
Truncated: no

```text
   Compiling astromech-voice v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-cmhv-names/crates/astromech-voice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.94s
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-bdbf9b5a9e39bc6c)
```

