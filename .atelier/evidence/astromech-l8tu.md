---
created_at: "2026-10-17T18:39:33.163094552+00:00"
id: "astromech-l8tu"
evidence_type: "test"
commit: "bf36a8f2492336e4f3a9be7e1b5af7bf3f4c055c"
result: "pass"
captured_at: "2026-10-17T18:39:24.483485113+00:00"
command: "cargo test -p astromech-tools -p astromech-keybinds"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-g1ys"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-g1ys"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-tools -p astromech-keybinds"
updated_at: "2026-10-17T18:39:33.163431611+00:00"
---

## Summary

cargo test -p astromech-tools -p astromech-keybinds

## Command

```console
cargo test -p astromech-tools -p astromech-keybinds
```

Exit status: 0

## Stdout

Bytes: 3498
Truncated: no

```text

running 25 tests
test catalog::tests::humanizes_ids ... ok
test binds::tests::file_names ... ok
test binds::tests::decode_variants ... ok
test binds::tests::malformed_position ... ok
test binds::tests::device_mappings ... ok
test active::tests::keyboard_bindings ... ok
test binds::tests::parse_sample ... ok
test binds::tests::keys ... ok
test binds::tests::start_preset ... ok
test devices::tests::axis_mapping ... ok
test devices::tests::fake_file ... ok
test game::tests::epic ... ok
test game::tests::vdf ... ok
test active::tests::resolves_ship_context ... ok
test game::tests::products ... ok
test edit::tests::adds_missing_actions_and_axes ... ok
test report::tests::redaction ... ok
test edit::tests::edits_only_the_slot ... ok
test doctor::tests::end_to_end ... ok
test catalog::tests::bundled_catalogue_loads ... ok
test profile::tests::no_keyboard_rule_covers_every_voice_action ... ok
test profile::tests::view_rows_and_issues ... ok
test profile::tests::device_and_pid_warnings ... ok
test edit::tests::file_edit_backup_restore_copy_and_activate ... ok
test api::tests::api_round_trip ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 33 tests
test colonisation::tests::commands ... ok
test faction::tests::every_lookup_has_a_command_and_a_spoken_noun ... ok
test faction::tests::nothing_found_says_so ... ok
test faction::tests::the_emissions_lookups_say_the_best_system_and_a_billion_people ... ok
test faction::tests::the_tester_says_what_it_would_look_up ... ok
test faction::tests::the_answer_says_where_how_far_and_how_old ... ok
test colonisation::tests::names_without_a_station ... ok
test colonisation::tests::the_project_is_the_one_docked_at_else_the_last_delivered_to ... ok
test colonisation::tests::progress_and_what_is_left ... ok
test session::tests::the_answer_is_one_sentence_per_fact ... ok
test session::tests::what_is_zero_is_left_out ... ok
test targets::tests::every_kind_has_a_command ... ok
test lookup::tests::service_slot_values_name_every_service ... ok
test targets::tests::nothing_found_says_so ... ok
test targets::tests::the_tester_says_what_it_would_look_up ... ok
test targets::tests::the_answer_says_where_how_far_and_the_state ... ok
test tests::commands_map ... ok
test tests::flight_assist_flag_is_inverted ... ok
test colonisation::tests::where_to_buy_needs_a_commodity_the_project_still_needs ... ok
test tests::gear_state_rules ... ok
test tests::health_check_covers_every_pressed_action ... ok
test tests::other_toggles ... ok
test tests::screens_and_throttle ... ok
test tests::silent_running_on_asks ... ok
test tests::powerplay_rank_is_answered_from_the_journal ... ok
test trade::tests::numbers_and_empty_answers_read_plainly ... ok
test tests::status_answers ... ok
test tests::system_powerplay_is_answered_from_the_last_jump ... ok
test tests::merits_here_names_the_first_activities ... ok
test lookup::tests::module_lookups_read_class_rating_and_mount_from_the_words ... ok
test lookup::tests::ship_answer_says_the_price ... ok
test lookup::tests::module_answer_says_the_price ... ok
test lookup::tests::spoken_answers_for_recorded_responses ... ok

test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 1167
Truncated: no

```text
   Compiling astromech-keybinds v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-keybinds)
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-data)
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-colonisation)
   Compiling astromech-input v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-input)
   Compiling astromech-policy v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-policy)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-tools)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.43s
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-926b772325f32f85)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-ecff1008b4af9a41)
   Doc-tests astromech_keybinds
   Doc-tests astromech_tools
```

