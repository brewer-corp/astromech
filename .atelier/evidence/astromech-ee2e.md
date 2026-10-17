---
created_at: "2026-10-17T17:16:38.182028403+00:00"
id: "astromech-ee2e"
evidence_type: "test"
commit: "0a2fa0a7fa68fc3ff2139226971de6423a7ebc75"
result: "pass"
captured_at: "2026-10-17T17:16:28.637449130+00:00"
command: "bash -c 'cargo test -p astromech-core --lib route && cargo test -p astromech-core --test gear_down && cargo test -p astromech-core --test copy && cargo test -p astromech-core --test voice_tester && cargo test -p astromech-intent route && cargo test -p astromech-core --lib controls::tests::paste'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-5sbu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-5sbu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Route follower behaviour: route.rs unit tests (follow, skip ahead, skip/back/clear, clipboard failure, restart, merge, CSV import), control API route commands, gear_down integration (FSDJump moves the route on, copies and announces the next; catch-up silent; paste only in the galaxy map sends Ctrl+V, Enter; voice route commands copy and speak), copy-guide check of the route lines, voice tester, grammar"
updated_at: "2026-10-17T17:16:38.182344533+00:00"
---

## Summary

Route follower behaviour: route.rs unit tests (follow, skip ahead, skip/back/clear, clipboard failure, restart, merge, CSV import), control API route commands, gear_down integration (FSDJump moves the route on, copies and announces the next; catch-up silent; paste only in the galaxy map sends Ctrl+V, Enter; voice route commands copy and speak), copy-guide check of the route lines, voice tester, grammar

## Command

```console
bash -c 'cargo test -p astromech-core --lib route && cargo test -p astromech-core --test gear_down && cargo test -p astromech-core --test copy && cargo test -p astromech-core --test voice_tester && cargo test -p astromech-intent route && cargo test -p astromech-core --lib controls::tests::paste'
```

Exit status: 0

## Stdout

Bytes: 3131
Truncated: no

```text

running 10 tests
test route::tests::merges_stops_in_one_system ... ok
test route::tests::follows_and_copies ... ok
test route::tests::skip_back_clear ... ok
test route::tests::parses_spansh_csv_and_plain_lists ... ok
test route::tests::clipboard_failure_is_reported ... ok
test route::tests::a_later_stop_skips_ahead ... ok
test route::tests::survives_a_restart ... ok
test app::tests::route_commands_over_the_control_api ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test callouts::tests::routes_each_channel_and_speaks_highest_priority_first ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 104 filtered out; finished in 0.01s


running 18 tests
test no_keyboard_binding_refuses_clearly ... ok
test jumping_into_a_waypoint_copies_and_announces_the_next ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test silent_running_asks_then_runs ... ok
test did_you_mean_then_yes_executes ... ok
test state_rules_press_nothing ... ok
test route_commands_copy_and_speak ... ok
test status_questions_are_spoken_not_pressed ... ok
test confirmation_is_logged_with_latency ... ok
test real_game_round_two ... ok
test did_you_mean_yes_learns_alias ... ok
test paste_only_in_the_galaxy_map ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test voice_path_with_fake_stt ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.53s


running 6 tests
test the_cargo_answer_limits_its_partial_buyer_sentences ... ok
test the_route_lines_keep_to_the_copy_guide ... ok
test the_colonisation_answers_keep_to_the_copy_guide ... ok
test the_powerplay_target_answers_keep_to_the_copy_guide ... ok
test the_faction_lookup_answers_keep_to_the_copy_guide ... ok
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 7 tests
test recorded_clip_shows_transcript_and_intent ... ignored, needs the voice worker, the speech model and a clip
test typed_phrase_without_elite_running_still_shows_keys ... ok
test route_commands_are_described ... ok
test kill_switch_during_the_live_run_wait_presses_nothing ... ok
test push_to_talk_in_test_mode_presses_nothing_until_the_page_closes ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 6 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s


running 1 test
test tests::route_phrases_execute_and_stay_apart ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.04s


running 1 test
test controls::tests::paste_button_fires_once_per_press ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 113 filtered out; finished in 0.00s
```

## Stderr

Bytes: 1339
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.88s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-7002ff8668dca506)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.67s
     Running tests/gear_down.rs (target/debug/deps/gear_down-453b17b31b0ab334)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running tests/copy.rs (target/debug/deps/copy-a7975326fa41c77f)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-route/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-ea8a5bfd84f289a1)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-49143e7857eb60de)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-7002ff8668dca506)
```

