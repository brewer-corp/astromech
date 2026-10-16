---
created_at: "2026-10-16T19:22:05.101426383+00:00"
id: "astromech-ogkf"
evidence_type: "test"
commit: "80f3f0fee3e26fc2784defc9b5a53f3dde2bcb74"
result: "pass"
captured_at: "2026-10-16T19:22:03.712379100+00:00"
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
title: "Re-review fixes (N2): kill switch pressed and re-armed during the live-run wait ends it with a kill-switch message and presses nothing; wait_foreground Front/TimedOut/Killed"
updated_at: "2026-10-16T19:22:05.113458822+00:00"
---

## Summary

Re-review fixes (N2): kill switch pressed and re-armed during the live-run wait ends it with a kill-switch message and presses nothing; wait_foreground Front/TimedOut/Killed

## Command

```console
cargo test -p astromech-core --test voice_tester -p astromech-policy
```

Exit status: 0

## Stdout

Bytes: 582
Truncated: no

```text

running 6 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test kill_switch_during_the_live_run_wait_presses_nothing ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.72s
```

## Stderr

Bytes: 158
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-0d54b9c565f8bbb4)
```

