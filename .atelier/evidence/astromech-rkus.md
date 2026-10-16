---
created_at: "2026-10-16T20:49:24.359129775+00:00"
id: "astromech-rkus"
evidence_type: "test"
commit: "b7f8ebf10ba90a43749644253f10789ef81d8c5f"
result: "pass"
captured_at: "2026-10-16T20:49:21.468867049+00:00"
command: "sh -c 'cargo test -p astromech-intent && cargo test -p astromech-core --test trade_voice && cargo test -p astromech-tools'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-sd56"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-sd56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Voice phrase tests: 'where should I sell my cargo' / 'sell my cargo' map to trade.sell_cargo; the core speaks the whole-load answer from the recorded hold and Ardent answers, uses the My cargo filters (partial take within 15 ly), and says when nothing is sellable; tester dry run"
updated_at: "2026-10-16T20:49:24.359400360+00:00"
---

## Summary

Voice phrase tests: 'where should I sell my cargo' / 'sell my cargo' map to trade.sell_cargo; the core speaks the whole-load answer from the recorded hold and Ardent answers, uses the My cargo filters (partial take within 15 ly), and says when nothing is sellable; tester dry run

## Command

```console
sh -c 'cargo test -p astromech-intent && cargo test -p astromech-core --test trade_voice && cargo test -p astromech-tools'
```

Exit status: 0

## Stdout

Bytes: 1897
Truncated: no

```text

running 13 tests
test tests::thresholds_are_the_documented_ones ... ok
test tests::noise_only_transcripts ... ok
test tests::normalisation ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::multi_word_system_names_fill_the_slot ... ok
test tests::s9_mic_misses ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::real_game_transcripts ... ok
test tests::v1_t1_and_status_phrases_execute ... ok
test tests::exact_phrases_execute ... ok
test tests::trade_phrases_fill_the_commodity_slot ... ok
test tests::misheard_commodities_ask ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test where_can_i_sell_uses_the_remembered_filters ... ok
test a_misheard_commodity_asks_first_then_answers ... ok
test where_can_i_buy_answers_the_cheapest_with_its_age ... ok
test where_should_i_sell_my_cargo_names_the_best_station_for_the_whole_load ... ok
test no_location_or_no_network_are_said_plainly ... ok
test the_voice_tester_says_what_it_would_look_up_without_asking_ardent ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.69s


running 8 tests
test tests::commands_map ... ok
test tests::flight_assist_flag_is_inverted ... ok
test tests::gear_state_rules ... ok
test tests::other_toggles ... ok
test tests::screens_and_throttle ... ok
test tests::silent_running_on_asks ... ok
test tests::status_answers ... ok
test trade::tests::numbers_and_empty_answers_read_plainly ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 655
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.36s
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-3d6d430bc259405e)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-a7a3c4fc79228bf6)
   Doc-tests astromech_tools
```

