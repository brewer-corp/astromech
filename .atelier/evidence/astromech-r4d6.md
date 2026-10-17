---
created_at: "2026-10-17T17:06:18.180543207+00:00"
id: "astromech-r4d6"
evidence_type: "test"
commit: "8383c2ac3e423e721d9f3a6d8eb6c24a9f3bc6f9"
result: "pass"
captured_at: "2026-10-17T17:06:17.039674975+00:00"
command: "cargo test -p astromech-gamestate -p astromech-tools"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-nw2d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-nw2d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-gamestate -p astromech-tools"
updated_at: "2026-10-17T17:06:18.180873362+00:00"
---

## Summary

cargo test -p astromech-gamestate -p astromech-tools

## Command

```console
cargo test -p astromech-gamestate -p astromech-tools
```

Exit status: 0

## Stdout

Bytes: 3417
Truncated: no

```text

running 5 tests
test tests::star_pos_follows_the_current_system ... ok
test tests::a_new_journal_file_clears_a_crashed_session_but_a_rollover_keeps_it ... ok
test tests::missions_follow_the_journal ... ok
test tests::logged_in_follows_the_login_and_the_logout ... ok
test tests::folds_events ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test events_map_to_files ... ok
test a_read_never_waits_for_a_file_the_game_has_not_written_yet ... ok
test station_market_id_follows_docking_login_and_undocking ... ok
test only_the_latest_event_of_a_kind_is_checked ... ok
test a_missing_file_is_reported ... ok
test a_half_written_file_is_reported ... ok
test a_file_older_than_its_event_is_reported_and_not_used ... ok
test each_event_leaves_its_snapshot_in_game_state ... ok
test a_stale_file_does_not_replace_the_snapshot_already_held ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test mining_fixture_gives_refined_count_and_tons_per_hour ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 32 tests
test colonisation::tests::commands ... ok
test faction::tests::every_lookup_has_a_command_and_a_spoken_noun ... ok
test faction::tests::nothing_found_says_so ... ok
test faction::tests::the_tester_says_what_it_would_look_up ... ok
test colonisation::tests::names_without_a_station ... ok
test colonisation::tests::the_project_is_the_one_docked_at_else_the_last_delivered_to ... ok
test colonisation::tests::progress_and_what_is_left ... ok
test faction::tests::the_emissions_lookups_say_the_best_system_and_a_billion_people ... ok
test targets::tests::every_kind_has_a_command ... ok
test faction::tests::the_answer_says_where_how_far_and_how_old ... ok
test session::tests::the_answer_is_one_sentence_per_fact ... ok
test targets::tests::the_tester_says_what_it_would_look_up ... ok
test session::tests::what_is_zero_is_left_out ... ok
test lookup::tests::service_slot_values_name_every_service ... ok
test tests::flight_assist_flag_is_inverted ... ok
test tests::gear_state_rules ... ok
test tests::other_toggles ... ok
test tests::screens_and_throttle ... ok
test targets::tests::the_answer_says_where_how_far_and_the_state ... ok
test targets::tests::nothing_found_says_so ... ok
test tests::commands_map ... ok
test trade::tests::numbers_and_empty_answers_read_plainly ... ok
test tests::powerplay_rank_is_answered_from_the_journal ... ok
test tests::status_answers ... ok
test tests::silent_running_on_asks ... ok
test tests::system_powerplay_is_answered_from_the_last_jump ... ok
test tests::merits_here_names_the_first_activities ... ok
test colonisation::tests::where_to_buy_needs_a_commodity_the_project_still_needs ... ok
test lookup::tests::module_lookups_read_class_rating_and_mount_from_the_words ... ok
test lookup::tests::ship_answer_says_the_price ... ok
test lookup::tests::module_answer_says_the_price ... ok
test lookup::tests::spoken_answers_for_recorded_responses ... ok

test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 576
Truncated: no

```text
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-bugs-state/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-e0ed561ab9490ce3)
     Running tests/companions.rs (target/debug/deps/companions-404713699029c353)
     Running tests/mining.rs (target/debug/deps/mining-1ba40a0a2429a61c)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-7da2f67dd6c7279c)
   Doc-tests astromech_gamestate
   Doc-tests astromech_tools
```

