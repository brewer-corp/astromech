---
created_at: "2026-10-16T20:54:28.745117892+00:00"
id: "astromech-efdt"
evidence_type: "test"
commit: "53fbc221b9282183e33c953f4a0409931f30fb62"
result: "pass"
captured_at: "2026-10-16T20:54:23.937806951+00:00"
command: "bash -c 'cargo test -p astromech-tools --locked lookup && cargo test -p astromech-intent --locked'"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-tw2y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tw2y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Spoken answers for recorded responses; intent multi-word slots"
updated_at: "2026-10-16T20:54:28.745510404+00:00"
---

## Summary

Spoken answers for recorded responses; intent multi-word slots

## Command

```console
bash -c 'cargo test -p astromech-tools --locked lookup && cargo test -p astromech-intent --locked'
```

Exit status: 0

## Stdout

Bytes: 1098
Truncated: no

```text

running 2 tests
test lookup::tests::service_slot_values_name_every_service ... ok
test lookup::tests::spoken_answers_for_recorded_responses ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.01s


running 13 tests
test tests::thresholds_are_the_documented_ones ... ok
test tests::class_and_rating_said_apart_stay_together ... ok
test tests::free_words_ride_along_in_the_slot_text ... ok
test tests::normalisation ... ok
test tests::noise_only_transcripts ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::s9_mic_misses ... ok
test tests::real_game_transcripts ... ok
test tests::multi_word_slots_resolve_to_their_value ... ok
test tests::v1_t1_and_status_phrases_execute ... ok
test tests::exact_phrases_execute ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 595
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.42s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-604fc48aee6d0e60)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
```

