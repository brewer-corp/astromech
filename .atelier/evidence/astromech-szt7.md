---
created_at: "2026-10-16T18:54:29.161522232+00:00"
id: "astromech-szt7"
evidence_type: "test"
commit: "d4d3633bac90d7667cea716d1974f4f8c11b42a2"
result: "pass"
captured_at: "2026-10-16T18:54:27.485341611+00:00"
command: "cargo test -p astromech-core --test voice_tester"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-fa3g"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-fa3g"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Typed phrase dry run shows intent and keys with nothing pressed (fake executor records no key events); PTT in test mode presses nothing until the lease lapses; live run presses once through the policy"
updated_at: "2026-10-16T18:54:29.161778100+00:00"
---

## Summary

Typed phrase dry run shows intent and keys with nothing pressed (fake executor records no key events); PTT in test mode presses nothing until the lease lapses; live run presses once through the policy

## Command

```console
cargo test -p astromech-core --test voice_tester
```

Exit status: 0

## Stdout

Bytes: 433
Truncated: no

```text

running 4 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok

test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.26s
```

## Stderr

Bytes: 277
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-q318/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.34s
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-0d54b9c565f8bbb4)
```

