---
created_at: "2026-10-16T20:42:49.711112375+00:00"
id: "astromech-9pw5"
evidence_type: "test"
commit: "9b67a356b32cfab02cd1165a90978e76bfc5185d"
result: "pass"
captured_at: "2026-10-16T20:42:35.691732700+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-powerplay && cargo test -p astromech-journal --test events system_powerplay && cargo test -p astromech-intent && cargo test -p astromech-tools system_powerplay && cd app && pnpm vitest run src/modules/powerplay'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-h1dz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-h1dz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Journal replay: an FSDJump into a fortified system of the pledged power, an enemy exploited system, an unoccupied acquisition system and a system without Powerplay give the expected arrival lines (none for Location and for no activity); the page summary for the last system; parser tests for the FSDJump/Location Powerplay fields; voice phrase and answer tests for 'who controls this system'"
updated_at: "2026-10-16T20:42:49.711339830+00:00"
---

## Summary

Journal replay: an FSDJump into a fortified system of the pledged power, an enemy exploited system, an unoccupied acquisition system and a system without Powerplay give the expected arrival lines (none for Location and for no activity); the page summary for the last system; parser tests for the FSDJump/Location Powerplay fields; voice phrase and answer tests for 'who controls this system'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-powerplay && cargo test -p astromech-journal --test events system_powerplay && cargo test -p astromech-intent && cargo test -p astromech-tools system_powerplay && cd app && pnpm vitest run src/modules/powerplay'
```

Exit status: 0

## Stdout

Bytes: 2143
Truncated: no

```text

running 2 tests
test rules::tests::rank_thresholds_follow_the_wiki_table ... ok
test rules::tests::the_cycle_turns_on_thursday_at_seven_utc ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test each_jump_gives_the_expected_arrival_line ... ok
test not_pledged_hears_the_state_without_a_target ... ok
test the_page_shows_the_current_system ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test merits_for_another_power_are_ignored ... ok
test joining_defecting_and_leaving ... ok
test replaying_the_journals_gives_the_standing ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test system_powerplay_on_jump_and_location ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s


running 10 tests
test tests::noise_only_transcripts ... ok
test tests::normalisation ... ok
test tests::thresholds_are_the_documented_ones ... ok
test tests::eval_counts_execute_or_ask ... ok
test tests::personal_alias_executes_next_time ... ok
test tests::s9_mic_misses ... ok
test tests::near_misses_ask_and_noise_rejects ... ok
test tests::real_game_transcripts ... ok
test tests::exact_phrases_execute ... ok
test tests::v1_t1_and_status_phrases_execute ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::system_powerplay_is_answered_from_the_last_jump ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-7ni2/app


 Test Files  1 passed (1)
      Tests  5 passed (5)
   Start at  16:42:48
   Duration  1.01s (environment 62%, setup 14%, transform 12%, tests 11%, import 1%)
```

## Stderr

Bytes: 1604
Truncated: no

```text
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-journal)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-powerplay)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.97s
     Running unittests src/lib.rs (target/debug/deps/astromech_powerplay-77f8159cfd0049fd)
     Running tests/arrival.rs (target/debug/deps/arrival-12bb1ca90a858bc3)
     Running tests/standing.rs (target/debug/deps/standing-935b1c16ddabc476)
   Doc-tests astromech_powerplay
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-journal)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.22s
     Running tests/events.rs (target/debug/deps/events-030d4df97a9204fd)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
   Doc-tests astromech_intent
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-materials)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-gamestate)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-7ni2/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.90s
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-f1470fe4bb020b1d)
```

