---
created_at: "2026-10-17T01:06:02.835918065+00:00"
id: "astromech-qdof"
evidence_type: "test"
commit: "b6ed86c8409f4b02d85992e59763626542f0e67b"
result: "pass"
captured_at: "2026-10-17T01:05:56.350780685+00:00"
command: "bash -c 'export CARGO_TARGET_DIR=$HOME/.cache/astromech-w62j/target; cargo test -p astromech-powerplay && cargo test -p astromech-tools merits_here && cargo test -p astromech-intent v1_t1_and_status_phrases_execute && cargo test -p astromech-core lookups'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-w62j"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-w62j"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Done-when fixture: every state in the rules table (own Stronghold/Fortified/Exploited, acquisition, conflict, enemy Exploited/Fortified/Stronghold incl. Delaine weak spot, other power's acquisition, no Powerplay) gets its expected ordered suggestions; table-consistency and conflict-edge unit tests; voice status.merits_here answer and intent phrases"
updated_at: "2026-10-17T01:06:02.836210661+00:00"
---

## Summary

Done-when fixture: every state in the rules table (own Stronghold/Fortified/Exploited, acquisition, conflict, enemy Exploited/Fortified/Stronghold incl. Delaine weak spot, other power's acquisition, no Powerplay) gets its expected ordered suggestions; table-consistency and conflict-edge unit tests; voice status.merits_here answer and intent phrases

## Command

```console
bash -c 'export CARGO_TARGET_DIR=$HOME/.cache/astromech-w62j/target; cargo test -p astromech-powerplay && cargo test -p astromech-tools merits_here && cargo test -p astromech-intent v1_t1_and_status_phrases_execute && cargo test -p astromech-core lookups'
```

Exit status: 0

## Stdout

Bytes: 2911
Truncated: no

```text

running 4 tests
test rules::tests::rank_thresholds_follow_the_wiki_table ... ok
test rules::tests::the_cycle_turns_on_thursday_at_seven_utc ... ok
test activities::tests::the_table_hangs_together ... ok
test activities::tests::conflict_needs_two_powers_past_the_threshold_with_mine_among_them ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test nothing_is_suggested_before_pledging ... ok
test suggestions_carry_the_details_the_page_shows ... ok
test each_state_in_the_rules_table_gets_its_suggestions ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test not_pledged_hears_the_state_without_a_target ... ok
test each_jump_gives_the_expected_arrival_line ... ok
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
test tests::merits_here_names_the_first_activities ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 1 test
test tests::v1_t1_and_status_phrases_execute ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.10s


running 6 tests
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test tester::tests::lookups_say_what_they_would_look_up ... ok
test lookups::tests::buy_goes_to_the_commodity_module_or_ship_the_slot_names ... ok
test lookups::tests::ship_phrases_resolve_to_lookups ... ok
test lookups::tests::module_phrases_resolve_to_lookups ... ok
test lookups::tests::service_phrases_resolve_to_lookups ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 2.74s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 1 test
test voice_site_kinds_are_the_galaxy_lookups ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s
```

## Stderr

Bytes: 3333
Truncated: no

```text
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-powerplay)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/astromech_powerplay-0b2dfeb7737c7dd6)
     Running tests/activities.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/activities-5c3897d6d5e5f6ef)
     Running tests/arrival.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/arrival-2184831b622cf546)
     Running tests/standing.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/standing-9c8386097323334d)
   Doc-tests astromech_powerplay
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/astromech_tools-c5aee6e79f35beac)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/astromech_intent-49143e7857eb60de)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-powerplay)
   Compiling astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-gamestate)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-galaxy)
   Compiling astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-station)
   Compiling astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-announce)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-w62j/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.13s
     Running unittests src/lib.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/astromech_core-6ce2077a9ea40510)
     Running unittests src/main.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/astromech_core-3b3c58feea08b5b9)
     Running tests/copy.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/copy-c75b178883e0ab3b)
     Running tests/gear_down.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/gear_down-7d478d23c11c17a9)
     Running tests/site_lookup.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/site_lookup-8c024b4048eed3b2)
     Running tests/trade_voice.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/trade_voice-c77e905fe2fbfc87)
     Running tests/voice_tester.rs (/home/supadava/.cache/astromech-w62j/target/debug/deps/voice_tester-82dd86370bcf8d2e)
```

