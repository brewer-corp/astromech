---
created_at: "2026-10-23T00:06:02.890473308+00:00"
id: "astromech-gblk"
evidence_type: "test"
commit: "e6ba59190363e27f8f94fe8c83512321f0e5d950"
result: "pass"
captured_at: "2026-10-23T00:05:52.940439493+00:00"
command: "cargo test -p astromech-core --test exploration_voice --test copy --test voice_eval_phrases --test colonisation_voice --test voice_tester"
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
title: "astromech-core exploration_voice: each question answered from a synthetic journal folder replay plus imported history (system value, worth mapping, biology left, unsold, return visit, no history), tester dry run; copy guide over longest answers; eval set covers every command. Seen failing (7 of 9) with the dispatch removed"
updated_at: "2026-10-23T00:06:02.890810436+00:00"
---

## Summary

astromech-core exploration_voice: each question answered from a synthetic journal folder replay plus imported history (system value, worth mapping, biology left, unsold, return visit, no history), tester dry run; copy guide over longest answers; eval set covers every command. Seen failing (7 of 9) with the dispatch removed

## Command

```console
cargo test -p astromech-core --test exploration_voice --test copy --test voice_eval_phrases --test colonisation_voice --test voice_tester
```

Exit status: 0

## Stdout

Bytes: 2957
Truncated: no

```text

running 8 tests
test the_project_last_delivered_to_wins_over_one_the_game_updated_later ... ok
test docked_at_a_site_the_answers_are_about_that_one ... ok
test progress_and_what_is_left_are_about_the_project_last_delivered_to ... ok
test where_to_buy_something_the_project_does_not_need_says_so ... ok
test the_loose_colonisation_phrasings_are_kept_as_they_are ... ok
test the_tester_shows_the_question_without_answering_it ... ok
test where_to_buy_runs_the_pages_lookup_with_its_remembered_filters ... ok
test colonisation_phrases_resolve_to_their_commands ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.10s


running 10 tests
test the_cargo_answer_limits_its_partial_buyer_sentences ... ok
test the_colonisation_answers_keep_to_the_copy_guide ... ok
test the_missing_key_help_lines_keep_to_the_copy_guide ... ok
test every_literal_the_core_says_keeps_to_the_copy_guide ... ok
test the_exploration_answers_keep_to_the_copy_guide ... ok
test the_route_lines_keep_to_the_copy_guide ... ok
test the_powerplay_target_answers_keep_to_the_copy_guide ... ok
test the_faction_lookup_answers_keep_to_the_copy_guide ... ok
test the_lookup_policy_and_service_lines_keep_to_the_copy_guide ... ok
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 9 tests
test the_system_value_now_and_with_the_best_mapped ... ok
test every_genus_analysed_says_so ... ok
test without_a_history ... ok
test unsold_data_from_the_history ... ok
test the_bodies_worth_mapping_best_first ... ok
test a_return_visit_answers_from_what_was_scanned_before ... ok
test the_biology_left_to_sample ... ok
test the_tester_shows_the_question_without_answering_it ... ok
test exploration_phrases_resolve_to_their_commands ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.02s


running 1 test
test phrase_set_covers_every_command ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test kill_switch_during_the_live_run_wait_presses_nothing ... ok
test typed_phrase_without_elite_running_still_shows_keys ... ok
test a_phrase_follows_the_mode_at_release_not_when_transcription_ends ... ok
test route_commands_are_described ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test a_learned_phrase_can_be_forgotten ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test genuine_phrases_are_learned_key_presses_after_two_yes ... ok
test ask_only_near_misses_are_never_learned ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 10 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s
```

## Stderr

Bytes: 635
Truncated: no

```text
   Compiling astromech-core v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-m3xd/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.82s
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-2b0ef61388cb7738)
     Running tests/copy.rs (target/debug/deps/copy-315b4194d7af2096)
     Running tests/exploration_voice.rs (target/debug/deps/exploration_voice-f5f34bf7260980f5)
     Running tests/voice_eval_phrases.rs (target/debug/deps/voice_eval_phrases-d6f67b189c296558)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-c607c943005347d8)
```

