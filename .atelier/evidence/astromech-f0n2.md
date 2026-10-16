---
created_at: "2026-10-16T20:39:35.072504571+00:00"
id: "astromech-f0n2"
evidence_type: "test"
commit: "391bf16e5c12553c6ad88914935f085d755a6c76"
result: "pass"
captured_at: "2026-10-16T20:39:26.727152821+00:00"
command: "sh -c 'cargo test -p astromech-intent && cargo test -p astromech-core --test trade_voice && cargo test -p astromech-tools'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-xqdl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xqdl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Voice phrase tests: grammar fills the commodity slot (multi-word, aliases, near names ask) and the core speaks the answer for recorded Ardent responses with remembered filters; tester dry run"
updated_at: "2026-10-16T20:39:35.073075227+00:00"
---

## Summary

Voice phrase tests: grammar fills the commodity slot (multi-word, aliases, near names ask) and the core speaks the answer for recorded Ardent responses with remembered filters; tester dry run

## Command

```console
sh -c 'cargo test -p astromech-intent && cargo test -p astromech-core --test trade_voice && cargo test -p astromech-tools'
```

Exit status: 0

## Stdout

Bytes: 1814
Truncated: no

```text

running 13 tests
test tests::noise_only_transcripts ... ok
test tests::normalisation ... ok
test tests::thresholds_are_the_documented_ones ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::multi_word_system_names_fill_the_slot ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::s9_mic_misses ... ok
test tests::real_game_transcripts ... ok
test tests::exact_phrases_execute ... ok
test tests::v1_t1_and_status_phrases_execute ... ok
test tests::misheard_commodities_ask ... ok
test tests::trade_phrases_fill_the_commodity_slot ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test where_can_i_sell_uses_the_remembered_filters ... ok
test a_misheard_commodity_asks_first_then_answers ... ok
test where_can_i_buy_answers_the_cheapest_with_its_age ... ok
test no_location_or_no_network_are_said_plainly ... ok
test the_voice_tester_says_what_it_would_look_up_without_asking_ardent ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s


running 8 tests
test tests::flight_assist_flag_is_inverted ... ok
test tests::status_answers ... ok
test tests::other_toggles ... ok
test tests::screens_and_throttle ... ok
test tests::gear_state_rules ... ok
test trade::tests::numbers_and_empty_answers_read_plainly ... ok
test tests::silent_running_on_asks ... ok
test tests::commands_map ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 1795
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.12s
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-d5fe5f3dfcd20b38)
   Compiling roxmltree v0.21.1
   Compiling astromech-proto v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-proto)
   Compiling astromech-status v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-status)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-gamestate)
   Compiling astromech-keybinds v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-keybinds)
   Compiling astromech-input v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-input)
   Compiling astromech-policy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-policy)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-zf3s/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.50s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-4ecbd7e69fbb3916)
   Doc-tests astromech_tools
```

