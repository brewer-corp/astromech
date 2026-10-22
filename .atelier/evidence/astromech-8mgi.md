---
created_at: "2026-10-22T21:30:33.207337714+00:00"
id: "astromech-8mgi"
evidence_type: "test"
commit: "dfcb12fd136d61438891084e3e8672dc52cabd6c"
result: "pass"
captured_at: "2026-10-22T21:30:28.084531862+00:00"
command: "cargo test -p astromech-gamestate --test mining -p astromech-journal --test events"
exit_status: "0"
agent_identity: "claude-implementer-fdh3"
target:
  kind: "issue"
  id: "astromech-gwh0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-gwh0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Repair/fuel/hatchbreaker/recon/research/decon limpets start no mining run, prospector and collector do; rate runs to the latest journal line (non-mining line after the last refine lowers t/h); LaunchDrone Type parsed"
updated_at: "2026-10-22T21:30:33.208105766+00:00"
---

## Summary

Repair/fuel/hatchbreaker/recon/research/decon limpets start no mining run, prospector and collector do; rate runs to the latest journal line (non-mining line after the last refine lowers t/h); LaunchDrone Type parsed

## Command

```console
cargo test -p astromech-gamestate --test mining -p astromech-journal --test events
```

Exit status: 0

## Stdout

Bytes: 1593
Truncated: no

```text

running 3 tests
test rate_runs_to_the_latest_journal_line_not_the_last_refine ... ok
test only_prospector_and_collector_limpets_start_a_run ... ok
test mining_fixture_gives_refined_count_and_tons_per_hour ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 30 tests
test a_scans_asteroid_belts_are_not_rings ... ok
test cargo_inventory ... ok
test docking_granted_and_denied ... ok
test every_fixture_line_is_typed_and_keeps_its_raw_json_byte_for_byte ... ok
test carrier_jump_stays_a_jump_and_keeps_the_dock ... ok
test fss_events ... ok
test codex_settlement_and_beacon ... ok
test exobiology_events ... ok
test bounty_and_voucher ... ok
test colonisation_family ... ok
test carrier_family ... ok
test market_outfitting_shipyard_point_at_the_station ... ok
test market_buy_and_sell ... ok
test combat_events ... ok
test raw_json_survives_odd_spacing_key_order_unicode_and_number_forms ... ok
test raw_json_is_kept_when_reading_a_journal_file ... ok
test saa_events ... ok
test jump_planning_events ... ok
test exploration_sales ... ok
test scan_body_and_barycentre ... ok
test unknown_events_still_fall_through_to_other ... ok
test ship_locker ... ok
test statistics_flattens_numeric_counters ... ok
test supercruise_transitions ... ok
test mining ... ok
test fuel_chat_and_surface_events ... ok
test system_powerplay_on_jump_and_location ... ok
test powerplay_family ... ok
test missions ... ok
test rank_progress_reputation ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Stderr

Bytes: 477
Truncated: no

```text
   Compiling astromech-powerplay v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-fdh3/crates/astromech-powerplay)
   Compiling astromech-gamestate v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-fdh3/crates/astromech-gamestate)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.79s
     Running tests/mining.rs (target/debug/deps/mining-2354c81d6a2aef7a)
     Running tests/events.rs (target/debug/deps/events-0c603fbbc8778377)
```

