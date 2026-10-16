---
created_at: "2026-10-16T19:55:13.799757911+00:00"
id: "astromech-9z5j"
evidence_type: "test"
commit: "49fd4da658c9b34062477dae76127643b814b925"
result: "pass"
captured_at: "2026-10-16T19:55:12.352500160+00:00"
command: "cargo test -p astromech-core --lib -- speech:: setup::tests::voice_settings_persist"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-orel"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-orel"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Changing the speaker changes the next reply's voice (TtsSpeaker with a fake Kokoro synth; cached lines aren't re-rendered) and the voice settings persist in astromech.toml across a reload; voice list is English, British first"
updated_at: "2026-10-16T19:55:13.800048464+00:00"
---

## Summary

Changing the speaker changes the next reply's voice (TtsSpeaker with a fake Kokoro synth; cached lines aren't re-rendered) and the voice settings persist in astromech.toml across a reload; voice list is English, British first

## Command

```console
cargo test -p astromech-core --lib -- speech:: setup::tests::voice_settings_persist
```

Exit status: 0

## Stdout

Bytes: 408
Truncated: no

```text

running 5 tests
test speech::tests::voice_choices_are_english_british_first ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test setup::tests::voice_settings_persist ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 50 filtered out; finished in 0.01s
```

## Stderr

Bytes: 278
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-lxma/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.36s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8c448b21aa4d9de9)
```

