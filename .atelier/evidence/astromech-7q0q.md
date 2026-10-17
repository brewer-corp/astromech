---
created_at: "2026-10-17T17:15:11.802845674+00:00"
id: "astromech-7q0q"
evidence_type: "test"
commit: "88794d19ae33784e0fe0764973f90bc2470784a6"
result: "pass"
captured_at: "2026-10-17T17:15:08.840817139+00:00"
command: "cargo test -p astromech-colonisation -p astromech-journal -p astromech-history"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-0x68"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-0x68"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-colonisation -p astromech-journal -p astromech-history"
updated_at: "2026-10-17T17:15:11.803586184+00:00"
---

## Summary

cargo test -p astromech-colonisation -p astromech-journal -p astromech-history

## Command

```console
cargo test -p astromech-colonisation -p astromech-journal -p astromech-history
```

Exit status: 0

## Stdout

Bytes: 5504
Truncated: yes

```text

running 20 tests
test tests::claim_without_address_keys_by_name_and_empty_events_are_tolerated ... ok
test tests::shopping_tests::commodity_ids_from_real_names ... ok
test tests::claims_and_beacons ... ok
test tests::replay_gives_progress_per_project ... ok
test tests::shopping_tests::dying_clears_the_hold ... ok
test tests::shopping_tests::cargo_without_inventory_keeps_the_hold ... ok
test tests::replaying_again_changes_nothing ... ok
test tests::shopping_tests::mid_session_events_move_the_hold ... ok
test tests::shopping_tests::carrier_transfers_move_cargo_on_and_off_the_ship ... ok
test tests::shopping_tests::mission_cargo_is_skipped_and_stolen_is_counted ... ok
test tests::shopping_tests::exact_multiple_extra_cargo_unknown_capacity_and_srv ... ok
test tests::state_is_kept_per_commander ... ok
test tests::shopping_tests::remaining_minus_cargo_in_loads ... ok
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok
test tests::stale_depot_does_not_overwrite_newer ... ok
test tests::save_writes_only_changes ... ok
test tests::commanders_persist_under_their_own_keys ... ok
test tests::shopping_tests::cargo_json_overrides_the_replayed_hold ... ok
test tests::unreadable_record_is_skipped_and_kept_and_the_rest_loads ... ok
test tests::survives_restart ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 28 tests
test exploration::tests::a_cached_poll_still_shows_the_earlier_bodies ... ok
test exploration::tests::a_system_visited_often_reads_one_scan_per_body ... ok
test tests::commanders_and_legacy_stay_apart ... ok
test exploration::tests::lines_since_last_reset_are_in_game_time_order ... ok
test exploration::tests::landings_count_flown_touchdowns_and_dropships_per_body ... ok
test exploration::tests::unsold_is_since_the_last_death_less_what_was_sold ... ok
test exploration::tests::the_system_view_shows_landings_and_the_first_footfall_hint ... ok
test exploration::tests::a_return_visit_shows_the_bodies_scanned_before_with_what_was_mapped ... ok
test exploration::tests::unsold_is_not_folded_again_until_the_database_changes ... ok
test exploration::tests::no_database_yet_is_no_commander_not_an_error ... ok
test tests::continuation_part_keeps_the_commander ... ok
test tests::import_reports_progress ... ok
test tests::earnings_ledger ... ok
test tests::logbook_search_finds_places_with_their_counts ... ok
test tests::imports_every_line_once ... ok
test tests::read_only_open_reads_what_the_writer_wrote ... ok
test tests::scans_ledger ... ok
test tests::rebuilt_ledgers_equal_imported_ones ... ok
test tests::ship_rows_ignore_older_events ... ok
test tests::ships_ledger ... ok
test tests::unfinished_line_waits_for_its_newline ... ok
test tests::session_summary_totals ... ok
test tests::visits_ledger ... ok
test tests::older_journal_imported_late_gives_the_same_ledgers ... ok
test tests::v1_database_is_migrated ... ok
test tests::import_killed_anywhere_then_resumed_equals_clean_import ... ok
test exploration::tests::system_lines_are_fast_with_many_signal_rows ... ok
test exploration::tests::landings_before_is_fast_with_a_large_history ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.01s


running 10 tests
test commander::tests::keys ... ok
test tests::cargo_and_route_files ... ok
test tests::docked_reads_what_the_station_offers ... ok
test tests::docked_keeps_system_address ... ok
test tests::jump_carries_factions_and_hge_signal ... ok
test tests::journal_names_order_across_schemes ... ok
test tests::known_and_unknown_events ... ok
test tests::materials_snapshot_and_deltas ... ok
test tests::positions_and_engineer_events ... ok
test tests::loadout_modules_and_fleet_events ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
test a_scans_asteroid_belts_are_not_rings ... ok
test cargo_inventory ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test docking_granted_and_denied ... ok
test every_fixture_line_
```

## Stderr

Bytes: 639
Truncated: no

```text
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-bugs-state/crates/astromech-colonisation)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-e94de3772014556a)
     Running unittests src/lib.rs (target/debug/deps/astromech_history-01dee2d8af9a2b52)
     Running unittests src/lib.rs (target/debug/deps/astromech_journal-e10d090a739a949c)
     Running tests/events.rs (target/debug/deps/events-f2c83d08d19466ae)
   Doc-tests astromech_colonisation
   Doc-tests astromech_history
   Doc-tests astromech_journal
```

