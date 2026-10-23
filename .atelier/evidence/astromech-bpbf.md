---
created_at: "2026-10-23T00:05:52.758875840+00:00"
id: "astromech-bpbf"
evidence_type: "test"
commit: "e6ba59190363e27f8f94fe8c83512321f0e5d950"
result: "pass"
captured_at: "2026-10-23T00:05:51.465431464+00:00"
command: "cargo test -p astromech-intent"
exit_status: "0"
agent_identity: "claude-implementer-m3xd"
target:
  kind: "issue"
  id: "astromech-m3xd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-m3xd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-intent: exploration phrases (incl. the five validation questions) execute, neighbours keep their commands, near misses never run another command; seen failing before the grammar entries"
updated_at: "2026-10-23T00:05:52.759235410+00:00"
---

## Summary

astromech-intent: exploration phrases (incl. the five validation questions) execute, neighbours keep their commands, near misses never run another command; seen failing before the grammar entries

## Command

```console
cargo test -p astromech-intent
```

Exit status: 0

## Stdout

Bytes: 1834
Truncated: no

```text

running 31 tests
test tests::class_and_rating_said_apart_stay_together ... ok
test tests::free_words_ride_along_in_the_slot_text ... ok
test tests::all_stop_fuzzy_near_misses_never_execute ... ok
test tests::a_sound_alike_of_a_personal_alias_asks ... ok
test tests::a_personal_alias_can_be_removed_and_a_builtin_phrase_cannot ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::noise_only_transcripts ... ok
test tests::normalisation ... ok
test tests::a_learned_neighbour_does_not_unlock_an_ask_only_phrase ... ok
test tests::pace_does_not_execute_back ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::ask_only_phrases_are_never_aliases ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::eval_mishearings ... ok
test tests::multi_word_slots_resolve_to_their_value ... ok
test tests::exact_phrases_execute ... ok
test tests::s9_mic_misses ... ok
test tests::slot_neighbours_do_not_execute ... ok
test tests::thresholds_are_the_documented_ones ... ok
test tests::round_four_neighbours_do_not_execute ... ok
test tests::route_phrases_execute_and_stay_apart ... ok
test tests::misheard_commodities_ask ... ok
test tests::sir_is_filler ... ok
test tests::real_game_transcripts ... ok
test tests::pays_asks_for_paste ... ok
test tests::sound_alikes_ask_and_never_execute ... ok
test tests::trade_phrases_fill_the_commodity_slot ... ok
test tests::all_stop_mishearings_ask_and_never_execute ... ok
test tests::exploration_phrases_execute_and_steal_nothing ... ok
test tests::v1_t1_and_status_phrases_execute ... ok
test tests::dictionary_sound_alikes_never_execute ... ok

test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.89s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 191
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-dd31d636140f577e)
   Doc-tests astromech_intent
```

