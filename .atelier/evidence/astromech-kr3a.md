---
created_at: "2026-10-16T20:03:56.158274888+00:00"
id: "astromech-kr3a"
evidence_type: "test"
commit: "f28ddf221fdd3339c2e0e6b2a19dbe868e824618"
result: "pass"
captured_at: "2026-10-16T20:03:55.438854068+00:00"
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
title: "Review fixes R1-R4 in core: replies render before the warm-up backlog and stale warm-ups are skipped (fails with warm-first ordering; 20/20 runs), PreviewSlot lifecycle (one Kokoro, refused while starting, idle release, speech-off error), next-reply and persistence tests still pass"
updated_at: "2026-10-16T20:03:56.158554781+00:00"
---

## Summary

Review fixes R1-R4 in core: replies render before the warm-up backlog and stale warm-ups are skipped (fails with warm-first ordering; 20/20 runs), PreviewSlot lifecycle (one Kokoro, refused while starting, idle release, speech-off error), next-reply and persistence tests still pass

## Command

```console
cargo test -p astromech-core --lib -- speech:: setup::tests::voice_settings_persist
```

Exit status: 0

## Stdout

Bytes: 523
Truncated: no

```text

running 7 tests
test setup::tests::voice_settings_persist ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test speech::tests::cache_names_are_stable ... ok
test speech::tests::preview_slot_lifecycle ... ok
test speech::tests::voice_choices_are_english_british_first ... ok
test speech::tests::set_voice_changes_the_next_reply ... ok
test speech::tests::replies_go_before_the_warm_up_backlog ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 50 filtered out; finished in 0.32s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8c448b21aa4d9de9)
```

