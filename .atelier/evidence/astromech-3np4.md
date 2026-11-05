---
created_at: "2026-11-04T23:20:42.449147380+00:00"
id: "astromech-3np4"
evidence_type: "test"
commit: "6fda4a5f52f9cbde3575d5946a68329ec542e540"
result: "fail"
captured_at: "2026-11-04T23:19:51.255040089+00:00"
command: "cargo test -p astromech-core -p astromech-journal"
exit_status: "1"
agent_identity: "implement-agent-ki3j"
target:
  kind: "issue"
  id: "astromech-73w9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-73w9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-core and astromech-journal suites pass at 6fda4a5f, including a_refused_touched_journals_header_is_read_once (five more looks read no header; fails with the cache disabled: 20 reads vs 5) and its replacement step (a renamed-in journal is read again and followed)"
updated_at: "2026-11-04T23:20:42.449470523+00:00"
---

## Summary

astromech-core and astromech-journal suites pass at 6fda4a5f, including a_refused_touched_journals_header_is_read_once (five more looks read no header; fails with the cache disabled: 20 reads vs 5) and its replacement step (a renamed-in journal is read again and followed)

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 1

## Stdout

Bytes: 24904
Truncated: yes
SHA-256: c6789ecead655decfdc8601c26eb58c06b50ce5ac2d694a64e8356a1eb6fdefd
Test summary: test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```text

running 197 tests
test app::tests::a_config_not_written_yet_gives_the_window_the_cores_folders ... ok
test app::tests::route_commands_over_the_control_api ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok
test app::tests::the_status_line_says_when_push_to_talk_is_only_testing ... ok
test app::tests::the_window_opens_its_cache_in_the_running_cores_data_folder ... ok
test autostart::tests::command_line_quotes_the_path ... ok
test app::tests::the_sample_config_written_by_open_config_changes_no_setting ... ok
test autostart::tests::state_round_trip ... ok
test app::tests::a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout ... ok
test app::tests::plotted_routes_load_into_the_follower ... ok
test background::tests::an_answer_is_looked_up_again_once_it_is_kept_for_long_enough ... ok
test background::tests::a_failing_lookup_is_retried_once_per_back_off_while_a_callout_waits ... ok
test biology::tests::a_recorded_answer_becomes_the_known_bodies ... ok
test calibration:
… 20808 bytes omitted …
o_yes ... ok
test typed_phrase_shows_intent_and_keys_with_nothing_pressed ... ok
test ask_only_near_misses_are_never_learned ... ok
test live_run_waits_for_elite_in_front_and_never_presses_into_another_window ... ok

test result: ok. 10 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.71s


running 14 tests
test tail::tests::a_file_that_shrinks_is_read_again_from_the_start ... ok
test tail::tests::read_to_stops_at_the_limit_and_read_new_takes_the_rest ... ok
test tests::cargo_and_route_files ... ok
test commander::tests::keys ... ok
test tests::known_and_unknown_events ... ok
test tests::docked_keeps_system_address ... ok
test tail::tests::tail_keeps_partial_lines ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::positions_and_engineer_events ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::docked_reads_what_the_station_offers ... ok
test tests::loadout_modules_and_fleet_events ... ok
test tests::the_latest_journal_is_by_the_stamp_in_its_name_and_never_a_conflict_copy ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
test a_scans_asteroid_belts_are_not_rings ... ok
test cargo_inventory ... ok
test exploration_sales ... ok
test codex_settlement_and_beacon ... ok
test bounty_and_voucher ... ok
test exobiology_events ... ok
test fss_events ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test docking_granted_and_denied ... ok
test combat_events ... ok
test market_buy_and_sell ... ok
test jump_planning_events ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test scan_body_and_barycentre ... ok
test saa_events ... ok
test colonisation_family ... ok
test mining ... ok
test rank_progress_reputation ... ok
test carrier_family ... ok
test unknown_events_still_fall_through_to_other ... ok
test supercruise_transitions ... ok
test powerplay_family ... ok
test fuel_chat_and_surface_events ... ok
test statistics_flattens_numeric_counters ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test ship_locker ... ok
test missions ... ok
test system_powerplay_on_jump_and_location ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 8 tests
test a_first_poll_that_cannot_open_the_journal_does_not_pin_the_build ... ok
test a_journal_that_shrinks_counts ... ok
test a_new_journal_file_counts ... ok
test a_new_journal_that_fails_to_open_is_counted_when_it_can_be_read ... ok
test a_cache_rebuilds_for_modelled_events_only ... ok
test a_half_written_line_counts_once_it_is_finished ... ok
test a_panicking_build_does_not_wedge_the_cache ... ok
test typed_appends_to_an_open_journal_change_the_generation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 11014
Truncated: yes
SHA-256: 0e5098a4e73f03c6fb46643eb352d9c6033735694cc38baed96bd62ccb64a3cc

```text
   Compiling astromech-journal v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-journal)
   Compiling astromech-materials v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-materials)
   Compiling astromech-exobiology v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-exobiology)
   Compiling astromech-powerplay v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-powerplay)
   Compiling astromech-colonisation v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-colonisation)
   Compiling astromech-odyssey v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-odyssey)
   Compiling astromech-engineering v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-engineering)
   Compiling astromech-exploration v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/astromech-exploration)
   Compiling astromech-gamestate v1.0.0-alpha.3 (/home/supadava/projects/astromech-wt3/crates/as
… 6918 bytes omitted …
/supadava/projects/astromech/target/debug/deps/libastromech_odyssey-ade95cfddc9bad86.rlib --extern astromech_policy=/home/supadava/projects/astromech/target/debug/deps/libastromech_policy-b114555e47fbcf64.rlib --extern astromech_powerplay=/home/supadava/projects/astromech/target/debug/deps/libastromech_powerplay-ea4d6d111e3ce128.rlib --extern astromech_proto=/home/supadava/projects/astromech/target/debug/deps/libastromech_proto-1246f8fa070ee29a.rlib --extern astromech_ship=/home/supadava/projects/astromech/target/debug/deps/libastromech_ship-b752090c8bfa95ee.rlib --extern astromech_station=/home/supadava/projects/astromech/target/debug/deps/libastromech_station-8e4bd6947397510c.rlib --extern astromech_status=/home/supadava/projects/astromech/target/debug/deps/libastromech_status-33e6309a2486cc17.rlib --extern astromech_store=/home/supadava/projects/astromech/target/debug/deps/libastromech_store-20968fcec0a34a3e.rlib --extern astromech_tools=/home/supadava/projects/astromech/target/debug/deps/libastromech_tools-f900b1d5a6b181af.rlib --extern astromech_voice=/home/supadava/projects/astromech/target/debug/deps/libastromech_voice-ba1c7fcc3c6040e3.rlib --extern chrono=/home/supadava/projects/astromech/target/debug/deps/libchrono-89a13fd68afe1d56.rlib --extern chrono_tz=/home/supadava/projects/astromech/target/debug/deps/libchrono_tz-1992d072e8aaf181.rlib --extern clap=/home/supadava/projects/astromech/target/debug/deps/libclap-4bb6deacfee7f89e.rlib --extern ed25519_dalek=/home/supadava/projects/astromech/target/debug/deps/libed25519_dalek-f6e9b0fe8e72f65c.rlib --extern flate2=/home/supadava/projects/astromech/target/debug/deps/libflate2-7df74d41d88f0370.rlib --extern jsonschema=/home/supadava/projects/astromech/target/debug/deps/libjsonschema-c7c6e1ac81f6b5e5.rlib --extern notify=/home/supadava/projects/astromech/target/debug/deps/libnotify-eeb25f422d3d1468.rlib --extern rusqlite=/home/supadava/projects/astromech/target/debug/deps/librusqlite-b25191e68d1dcf7f.rlib --extern serde=/home/supadava/projects/astromech/target/debug/deps/libserde-fd35ff3c75fc14d1.rlib --extern serde_json=/home/supadava/projects/astromech/target/debug/deps/libserde_json-4de127997d6d85b7.rlib --extern sha2=/home/supadava/projects/astromech/target/debug/deps/libsha2-2fc502fc5c28e3c8.rlib --extern tempfile=/home/supadava/projects/astromech/target/debug/deps/libtempfile-16d263abcbc5fbbf.rlib --extern toml=/home/supadava/projects/astromech/target/debug/deps/libtoml-ba766af2a18ced11.rlib --extern toml_edit=/home/supadava/projects/astromech/target/debug/deps/libtoml_edit-2e669290ba42b58b.rlib --extern ureq=/home/supadava/projects/astromech/target/debug/deps/libureq-922bc0cbf02ceee3.rlib --extern zip=/home/supadava/projects/astromech/target/debug/deps/libzip-4741ff92663a53e0.rlib -L dependency=/home/supadava/projects/astromech/target/debug/deps -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' --error-format human` (exit status: 1)
note: test exited abnormally; to see the full output pass --no-capture to the harness.
```
