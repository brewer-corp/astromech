---
created_at: "2026-10-17T00:03:39.697348360+00:00"
id: "astromech-aq0k"
evidence_type: "test"
commit: "e3cd7deb8351358554dcfecde039bb0fd64e9a96"
result: "pass"
captured_at: "2026-10-17T00:03:38.601618052+00:00"
command: "cargo test -p astromech-eddn --lib -- the_queue_is_capped discarding_the_queue switched_off a_companion_file_arriving"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6xyo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6xyo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R4: one outbox-full row per overflow episode, discard_all logs once; R1 note: stale companion file not sent (each fails without its fix)"
updated_at: "2026-10-17T00:03:39.697630577+00:00"
---

## Summary

R4: one outbox-full row per overflow episode, discard_all logs once; R1 note: stale companion file not sent (each fails without its fix)

## Command

```console
cargo test -p astromech-eddn --lib -- the_queue_is_capped discarding_the_queue switched_off a_companion_file_arriving
```

Exit status: 0

## Stdout

Bytes: 607
Truncated: no

```text

running 6 tests
test outbox::tests::discarding_the_queue_empties_it_and_logs_once ... ok
test worker::tests::switched_off_it_posts_nothing_and_discards_the_queue ... ok
test feed::tests::switched_off_a_pending_run_or_companion_file_queues_nothing ... ok
test feed::tests::a_companion_file_arriving_after_its_event_went_stale_is_not_sent ... ok
test feed::tests::switched_off_nothing_is_queued_and_switching_on_resumes_with_the_right_context ... ok
test outbox::tests::the_queue_is_capped_by_row_count ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 101 filtered out; finished in 0.18s
```

## Stderr

Bytes: 275
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-6xyo/crates/astromech-eddn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-c3e02632dd7d9212)
```

