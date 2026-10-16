---
created_at: "2026-10-16T18:54:40.695432718+00:00"
id: "astromech-p7pv"
evidence_type: "test"
commit: "d4d3633bac90d7667cea716d1974f4f8c11b42a2"
result: "pass"
captured_at: "2026-10-16T18:54:35.196044129+00:00"
command: "cargo test -p astromech-core --test voice_tester recorded_clip -- --ignored --nocapture"
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
title: "Recorded clip (synthetic Kokoro v0.19 'Gear down.' WAV, not committed) through the real voice worker + Parakeet TDT v2 int8 and the push-to-talk test-mode path: transcript 'Gear down.', intent landing_gear.down 100% execute, keys Key_L, nothing pressed. Linux reference run; the Windows mic/PTT capture is not exercised"
updated_at: "2026-10-16T18:54:40.695705787+00:00"
---

## Summary

Recorded clip (synthetic Kokoro v0.19 'Gear down.' WAV, not committed) through the real voice worker + Parakeet TDT v2 int8 and the push-to-talk test-mode path: transcript 'Gear down.', intent landing_gear.down 100% execute, keys Key_L, nothing pressed. Linux reference run; the Windows mic/PTT capture is not exercised

## Command

```console
cargo test -p astromech-core --test voice_tester recorded_clip -- --ignored --nocapture
```

Exit status: 0

## Stdout

Bytes: 407
Truncated: no

```text

running 1 test
clip /tmp/fa3g-clip/gear-down.wav: heard "Gear down." -> Some(Intent { command: "landing_gear.down", phrase: "gear down", score: 1.0, decision: "execute" }) would Press, keys Some(KeyCombo { key: "Key_L", modifiers: [], slot: "Secondary" })
test recorded_clip_shows_transcript_and_intent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 5.33s
```

## Stderr

Bytes: 158
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-0d54b9c565f8bbb4)
```

