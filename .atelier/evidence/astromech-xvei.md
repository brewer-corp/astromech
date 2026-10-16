---
created_at: "2026-10-16T21:05:07.819027784+00:00"
id: "astromech-xvei"
evidence_type: "test"
commit: "710312cc432f4bb877c1a0127872134b0ee51152"
result: "pass"
captured_at: "2026-10-16T21:04:59.227880148+00:00"
command: "bash -c 'cargo test -p astromech-core --locked lookups && cargo test -p astromech-tools --locked lookup && cargo test -p astromech-intent --locked'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-puas"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-puas"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Voice phrase tests for ships (10 phrases incl. marks), spoken answer with price, intent mark normalisation"
updated_at: "2026-10-16T21:05:07.819375864+00:00"
---

## Summary

Voice phrase tests for ships (10 phrases incl. marks), spoken answer with price, intent mark normalisation

## Command

```console
bash -c 'cargo test -p astromech-core --locked lookups && cargo test -p astromech-tools --locked lookup && cargo test -p astromech-intent --locked'
```

Exit status: 0

## Stdout

Bytes: 2074
Truncated: no

```text

running 5 tests
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test tester::tests::lookups_say_what_they_would_look_up ... ok
test lookups::tests::ship_phrases_resolve_to_lookups ... ok
test lookups::tests::module_phrases_resolve_to_lookups ... ok
test lookups::tests::service_phrases_resolve_to_lookups ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.68s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s


running 5 tests
test lookup::tests::service_slot_values_name_every_service ... ok
test lookup::tests::module_lookups_read_class_rating_and_mount_from_the_words ... ok
test lookup::tests::module_answer_says_the_price ... ok
test lookup::tests::ship_answer_says_the_price ... ok
test lookup::tests::spoken_answers_for_recorded_responses ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.01s


running 13 tests
test tests::class_and_rating_said_apart_stay_together ... ok
test tests::normalisation ... ok
test tests::noise_only_transcripts ... ok
test tests::thresholds_are_the_documented_ones ... ok
test tests::free_words_ride_along_in_the_slot_text ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::s9_mic_misses ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::multi_word_slots_resolve_to_their_value ... ok
test tests::real_game_transcripts ... ok
test tests::exact_phrases_execute ... ok
test tests::v1_t1_and_status_phrases_execute ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 1122
Truncated: no

```text
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.79s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-9ead6ca7f9b9a7b4)
     Running unittests src/main.rs (target/debug/deps/astromech_core-1620298b589ad367)
     Running tests/gear_down.rs (target/debug/deps/gear_down-f08141c738926b64)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-dff127e51d0f749e)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.42s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-604fc48aee6d0e60)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
```

