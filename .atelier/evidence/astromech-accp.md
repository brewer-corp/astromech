---
created_at: "2026-10-22T20:56:58.497545613+00:00"
id: "astromech-accp"
evidence_type: "test"
commit: "500d718b7fcec6cc234d50d40bafc56ad7d08e44"
result: "pass"
captured_at: "2026-10-22T20:56:54.529026547+00:00"
command: "cargo test -p astromech-core --lib -- background biology known_bodies"
exit_status: "0"
agent_identity: "claude-implementer-g3qt"
target:
  kind: "issue"
  id: "astromech-54q6"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-54q6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Background tests with a fake clock: a failing lookup asked every second for the 60 s window is tried 5 times (0/15/30/45/60 s); a Done answer is looked up again at KEEP_FOR (1 h). Both fail under mutation (no expiry; 10 s back-off)"
updated_at: "2026-10-22T20:56:58.497995351+00:00"
---

## Summary

Background tests with a fake clock: a failing lookup asked every second for the 60 s window is tried 5 times (0/15/30/45/60 s); a Done answer is looked up again at KEEP_FOR (1 h). Both fail under mutation (no expiry; 10 s back-off)

## Command

```console
cargo test -p astromech-core --lib -- background biology known_bodies
```

Exit status: 0

## Stdout

Bytes: 776
Truncated: no

```text

running 8 tests
test known_bodies::tests::a_recorded_answer_is_bodies_against_the_count ... ok
test background::tests::an_answer_is_looked_up_again_once_it_is_kept_for_long_enough ... ok
test biology::tests::the_view_lists_bodies_known_and_sampling ... ok
test biology::tests::a_recorded_answer_becomes_the_known_bodies ... ok
test background::tests::a_failing_lookup_is_retried_once_per_back_off_while_a_callout_waits ... ok
test biology::tests::the_first_ask_starts_a_lookup_and_later_asks_answer ... ok
test biology::tests::a_failed_lookup_is_unknown_not_nothing_and_is_tried_again ... ok
test callouts::tests::notifications_run_in_the_background_at_most_one_per_gap ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 1.52s
```

## Stderr

Bytes: 219
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-badbc78a43d6af96)
```

