---
created_at: "2026-10-16T19:06:23.319137251+00:00"
id: "astromech-tgr6"
evidence_type: "test"
commit: "327fa7f84229de662b1b82a2fe39871e2fa691c0"
result: "pass"
captured_at: "2026-10-16T19:06:21.774819710+00:00"
command: "cargo test -p astromech-core --test voice_tester -p astromech-policy"
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
title: "Review fixes (R1): live run waits for Elite in front and presses nothing into another window (fake executor reports astromech active); typed dry run and PTT test mode still press nothing"
updated_at: "2026-10-16T19:06:23.326500114+00:00"
---

## Summary

Review fixes (R1): live run waits for Elite in front and presses nothing into another window (fake executor reports astromech active); typed dry run and PTT test mode still press nothing

## Command

```console
cargo test -p astromech-core --test voice_tester -p astromech-policy
```

Exit status: 0

## Stdout

Bytes: 517
Truncated: no

```text

running 5 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.70s
```

## Stderr

Bytes: 277
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-q318/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.75s
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-0d54b9c565f8bbb4)
```

