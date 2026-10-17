---
created_at: "2026-10-16T23:59:38.482331349+00:00"
id: "astromech-za1x"
evidence_type: "test"
commit: "57ffa6c7dfc826bbe8be49ae0bdcd74db4ff275e"
result: "pass"
captured_at: "2026-10-16T23:59:36.652859330+00:00"
command: "cargo test -p astromech-core --lib lazy_speaker"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mcdt"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mcdt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "LazySpeaker: model missing at start prints lines; once find_tts-style ready signal holds, next say starts TtsSpeaker (fake synth) once and speaks it plus queued lines in order in the voice chosen meanwhile. Fails (times out) if the speaker never starts."
updated_at: "2026-10-16T23:59:38.482626520+00:00"
---

## Summary

LazySpeaker: model missing at start prints lines; once find_tts-style ready signal holds, next say starts TtsSpeaker (fake synth) once and speaks it plus queued lines in order in the voice chosen meanwhile. Fails (times out) if the speaker never starts.

## Command

```console
cargo test -p astromech-core --lib lazy_speaker
```

Exit status: 0

## Stdout

Bytes: 184
Truncated: no

```text

running 1 test
test speech::tests::lazy_speaker_starts_once_the_model_is_ready ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 71 filtered out; finished in 0.08s
```

## Stderr

Bytes: 275
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mcdt/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.61s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-094d1cbbf733166e)
```

