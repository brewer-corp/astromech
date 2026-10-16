---
created_at: "2026-10-16T19:28:39.391701374+00:00"
id: "astromech-pmxn"
evidence_type: "test"
commit: "75aaae2fbb5e6f8d9b8a2a4185445b58c1c9e398"
result: "pass"
captured_at: "2026-10-16T19:28:26.605810495+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build'"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-qz9u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qz9u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Pre-PR checks after the review fixes: fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, app lint/check/build. Shell not compiled on Linux."
updated_at: "2026-10-16T19:28:39.391945831+00:00"
---

## Summary

Pre-PR checks after the review fixes: fmt --check, clippy -D warnings, cargo test --workspace, check-deps, shell cargo metadata --locked --offline, app lint/check/build. Shell not compiled on Linux.

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && (cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null) && cd app && pnpm lint && pnpm check && pnpm build'
```

Exit status: 0

## Stdout

Bytes: 19752
Truncated: yes

```text

running 28 tests
test autostart::tests::command_line_quotes_the_path ... ok
test autostart::tests::state_round_trip ... ok
test config::tests::finds_look_alike_config_names ... ok
test controls::tests::first_report_is_the_ptt_press ... ok
test controls::tests::debug_input_logs_every_button ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test gamewatch::tests::game_without_launcher ... ok
test gamewatch::tests::launcher_game_exit ... ok
test controls::tests::bounce_gives_one_clip ... ok
test gamewatch::tests::finds_newest_journal ... ok
test controls::tests::ptt_after_axis_traffic_and_kill_switch ... ok
test pipeline::tests::tail_keeps_partial_lines ... ok
test speech::tests::cache_names_are_stable ... ok
test update::tests::baked_public_key_parses ... ok
test logging::tests::rolls_over_and_prunes ... ok
test health::tests::reports_missing_keyboard_binding ... ok
test update::tests::version_order ... ok
test update::tests::copy_skips_data_dir ... ok
test controls::tests::replay_fixture_reaches_ptt ... ok
test speech::gate_tests::gates_short_quiet_and_passes_speech ... ok
test controls::tests::describes_missing_and_implausible_buttons ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test diagnostics::tests::bundles_logs_and_reports ... ok
test crash::tests::marker_round_trip ... ok
test gamewatch::tests::lists_processes ... ok
test control::tests::round_trip_and_token ... ok
test eval::tests::loads_sidecars_and_labels ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 15 tests
test silent_running_asks_then_runs ... ok
test no_keyboard_binding_refuses_clearly ... ok
test night_vision_without_keyboard_key_is_refused_clearly ... ok
test status_questions_are_spoken_not_pressed ... ok
test did_you_mean_then_yes_executes ... ok
test state_rules_press_nothing ... ok
test real_game_round_two ... ok
test did_you_mean_yes_learns_alias ... ok
test voice_path_with_fake_stt ... ok
test confirmation_is_logged_with_latency ... ok
test duplicate_is_suppressed_while_in_flight ... ok
test journal_rollover_and_saved_positions ... ok
test gear_down_success_across_the_measured_delay_range ... ok
test missed_press_is_logged_not_retried_or_spoken ... ok
test kill_switch_cancels_and_blocks_until_rearmed ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.52s


running 3 tests
test limit::tests::rate_limit_spaces_requests_per_service ... ok
test transport::tests::segments_and_cache_keys ... ok
test spansh::tests::search_body_uses_verified_shapes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 12 tests
test uncached_calls_always_ask ... ok
test spansh_route_job_round_trip ... ok
test offline_still_serves_a_fresh_cached_answer ... ok
test spansh_system_names_autocomplete ... ok
test edsm_endpoints_parse_recorded_answers ... ok
test http_errors_are_plain_words ... ok
test no_network_gives_a_plain_words_error ... ok
test ardent_endpoints_parse_recorded_answers ... ok
test spansh_searches_parse_recorded_answers ... ok
test cache_survives_a_restart_and_ttls_differ_by_kind ... ok
test schema_mismatch_is_an_error_naming_the_service ... ok
test cached_answer_is_served_without_a_request_until_ttl_ends ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 11 tests
test tests::trades_and_farm_respect_storage_caps ... ok
test tests::farm_route_trades_down_from_anchors ... ok
test tests::trader_rates ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::fleet_from_history ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::catalogue_materials_resolve ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::experimenta
```

## Stderr

Bytes: 3879
Truncated: no

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-qz9u/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-feature-astromech-qz9u/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-qz9u/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-b6c00dba852f455e)
     Running unittests src/main.rs (target/debug/deps/astromech_core-ce64816f2e4fd3ec)
     Running tests/gear_down.rs (target/debug/deps/gear_down-535ee79e56361fb4)
     Running unittests src/lib.rs (target/debug/deps/astromech_data-85b9b081ec1c75ec)
     Running tests/recorded.rs (target/debug/deps/recorded-b5ec8213d366a513)
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-47de08a4b04b1860)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-4d6019b18242c6a6)
     Running unittests src/lib.rs (target/debug/deps/astromech_gamestate-63ba36b87762ef71)
     Running unittests src/lib.rs (target/debug/deps/astromech_input-8c30d105fc6d1015)
     Running tests/listen.rs (target/debug/deps/listen-60b2d3a5beaad99d)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-9331dbbdc7bbec35)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-0aadfc2f5d71280c)
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-514a54a502c408af)
     Running unittests src/lib.rs (target/debug/deps/astromech_materials-8416b38c54a0aceb)
     Running unittests src/lib.rs (target/debug/deps/astromech_overlay-c73768c8bd01a11f)
     Running unittests src/lib.rs (target/debug/deps/astromech_policy-d7bc19546b5b1b63)
     Running unittests src/lib.rs (target/debug/deps/astromech_proto-f4c6dcf9c8519f66)
     Running unittests src/lib.rs (target/debug/deps/astromech_status-98b02c29099b2c2d)
     Running unittests src/lib.rs (target/debug/deps/astromech_store-6dfddf9596e437bb)
     Running unittests src/lib.rs (target/debug/deps/astromech_tools-27fbc016e30e0fd2)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-807f84173a65359d)
   Doc-tests astromech_core
   Doc-tests astromech_data
   Doc-tests astromech_engineering
   Doc-tests astromech_galaxy
   Doc-tests astromech_gamestate
   Doc-tests astromech_input
   Doc-tests astromech_intent
   Doc-tests astromech_journal
   Doc-tests astromech_keybinds
   Doc-tests astromech_materials
   Doc-tests astromech_overlay
   Doc-tests astromech_policy
   Doc-tests astromech_proto
   Doc-tests astromech_status
   Doc-tests astromech_store
   Doc-tests astromech_tools
   Doc-tests astromech_voice
```

