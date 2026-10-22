---
created_at: "2026-10-22T22:12:06.595166908+00:00"
id: "astromech-9pgu"
evidence_type: "test"
commit: "21e7add372f503d5323f41c13760c817f4f152c9"
result: "pass"
captured_at: "2026-10-22T22:12:03.374520127+00:00"
command: "cargo test -p astromech-core --lib speech::"
exit_status: "0"
agent_identity: "claude-implementer-uuqt"
target:
  kind: "issue"
  id: "astromech-550b"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-550b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "speech tests pass: closing_the_speaker_stops_rendering (slow gated synth, 20 warm-ups and a reply queued; after drop at most the in-progress line renders and the synth is dropped) and the deterministic panic test with a late-queued line (cmhv review R1/R2); both fail against the old loop (21 renders after close; busy stuck). 5 repeat runs green."
updated_at: "2026-10-22T22:12:06.595516871+00:00"
---

## Summary

speech tests pass: closing_the_speaker_stops_rendering (slow gated synth, 20 warm-ups and a reply queued; after drop at most the in-progress line renders and the synth is dropped) and the deterministic panic test with a late-queued line (cmhv review R1/R2); both fail against the old loop (21 renders after close; busy stuck). 5 repeat runs green.

## Command

```console
cargo test -p astromech-core --lib speech::
```

Exit status: 0

## Stdout

Bytes: 847
Truncated: no

```text

running 11 tests
test speech::tests::busy_while_a_line_renders_or_plays_without_a_device ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::preview_slot_lifecycle ... ok
test speech::tests::a_panicking_renderer_does_not_leave_the_speaker_busy ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test speech::tests::closing_the_speaker_stops_rendering ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test speech::tests::tts_speaker_busy_through_render_and_after_a_panic ... ok
test speech::tests::lazy_speaker_starts_once_the_model_is_ready ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 156 filtered out; finished in 0.28s
```

## Stderr

Bytes: 434
Truncated: no

```text
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling astromech-core v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-uuqt/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.70s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-c1154ff8594e4206)
```

