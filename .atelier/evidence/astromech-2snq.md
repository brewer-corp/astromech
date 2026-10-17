---
created_at: "2026-10-17T21:51:57.003517752+00:00"
id: "astromech-2snq"
evidence_type: "test"
commit: "ec3e2dcca322808613ccb3e8852998a584a89873"
result: "pass"
captured_at: "2026-10-17T21:49:51.123559672+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && python3 scripts/check-guide.py && cargo deny check && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-yil0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yil0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && python3 scripts/check-guide.py && cargo deny check && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'"
updated_at: "2026-10-17T21:51:57.004366926+00:00"
---

## Summary

bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && python3 scripts/check-guide.py && cargo deny check && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && python3 scripts/check-deps.py && python3 scripts/check-guide.py && cargo deny check && cd app && pnpm install --frozen-lockfile && pnpm lint && pnpm check && pnpm build && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 90473
Truncated: yes

```text

running 8 tests
test known_bodies::tests::lines ... ok
test prospector::tests::thresholds_default_override_and_off ... ok
test session_summary::tests::credits_round_into_millions_at_the_boundary ... ok
test catalog_tests::every_built_in_rule_has_a_title_and_a_default ... ok
test tests::queue_orders_by_priority_then_age_and_drops_stale_and_low ... ok
test route_follow::tests::only_live_arrivals_move_the_route ... ok
test route_follow::tests::the_toggle_turns_following_off_but_not_where_you_are ... ok
test tests::settings_read_from_toml ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 16 tests
test a_new_system_says_nothing_about_visits ... ok
test a_system_visited_twice_before_says_so_and_when_last ... ok
test how_long_ago_reads_in_the_nearest_unit ... ok
test landings_lines ... ok
test a_visited_fully_known_system_with_nothing_notable_gets_nothing ... ok
test a_state_the_shopping_list_does_not_want_gets_nothing ... ok
test a_material_collected_up_to_its_count_drops_off_the_list ... ok
test several_bodies_are_counted_and_none_says_nothing ... ok
test a_system_whose_state_the_shopping_list_wants_gets_its_state_line ... ok
test a_body_landed_on_before_is_named_with_how_often ... ok
test star_class_security_and_allegiance_are_never_spoken ... ok
test the_visits_line_follows_the_state_line_within_the_word_budget ... ok
test an_undiscovered_system_is_said_after_the_jump ... ok
test the_briefing_is_silent_during_the_catch_up ... ok
test the_pledged_power_hears_undermining_and_acquisition_targets ... ok
test the_visits_line_is_silent_in_the_catch_up_and_without_a_history ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
test can_be_turned_off ... ok
test a_late_answer_awaited_in_one_system_is_not_said_in_the_next ... ok
test a_relog_into_a_system_already_scanned_says_nothing ... ok
test values_and_lines_read_well ... ok
test arrival_with_known_biology_says_it_before_any_scan ... ok
test a_late_answer_is_said_on_the_tick_within_a_minute_and_once ... ok
test a_carrier_jump_or_a_login_is_an_arrival_too ... ok
test each_system_starts_afresh ... ok
test the_summary_follows_the_fss_and_the_dss ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 4 tests
test it_can_be_turned_off ... ok
test an_earth_like_world_gets_one_line_and_all_bodies_found_none ... ok
test nav_beacon_data_is_not_a_discovery_and_each_visit_starts_afresh ... ok
test terraformables_and_mapping_targets_are_called_out_and_the_rest_are_not ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 4 tests
test docking_callouts_are_silent_during_the_catch_up ... ok
test each_denial_reason_gets_its_line ... ok
test orbis_and_ocellus_share_the_layout_others_get_nothing ... ok
test a_coriolis_pad_number_gives_its_position ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test enough_fuel_gives_no_warning ... ok
test fuel_callouts_are_silent_during_the_catch_up ... ok
test another_ship_starts_without_a_fuel_estimate ... ok
test not_enough_fuel_to_reach_a_scoopable_star_warns_once ... ok
test a_neutron_route_does_not_hear_it_every_other_jump ... ok
test the_next_scoopable_star_is_said_once_per_stretch_without_one ... ok
test a_boosted_jump_does_not_cheapen_the_fuel_estimate ... ok
test off_the_route_or_before_any_jump_nothing_is_said ... ok
test at_a_scoopable_star_short_of_the_next_one_it_says_scoop_here ... ok
test no_scoopable_star_left_is_said_once_per_arrival ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test hge_callouts_are_silent_during_the_catch_up ... ok
test an_hge_in_a_famine_system_names_the_spawning_factions_pool ... ok
test other_signals_stay_quiet ... ok
test a_burst_of_hges_from_one_honk_is_said_once ... ok
test a_billion_people_system_for_a_wanted_pool_material_is_a_good_
```

## Stderr

Bytes: 13708
Truncated: yes

```text
    Checking astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-journal)
    Checking astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-materials)
    Checking astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-powerplay)
    Checking astromech-exobiology v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-exobiology)
    Checking astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-colonisation)
    Checking astromech-exploration v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-exploration)
    Checking astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-engineering)
    Checking astromech-odyssey v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-odyssey)
    Checking astromech-gamestate v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-gamestate)
    Checking astromech-history v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-history)
    Checking astromech-announce v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-announce)
    Checking astromech-ship v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-ship)
    Checking astromech-station v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-station)
    Checking astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-galaxy)
    Checking astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-market)
    Checking astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-tools)
    Checking astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.61s
warning: output filename collision at /home/supadava/projects/astromech-wt-voiceeval/target/debug/examples/dump-fixture
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
warning: output filename collision at /home/supadava/projects/astromech-wt-voiceeval/target/debug/examples/dump-fixture.dwp
  |
  = note: the example target `dump-fixture` in package `astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-materials)` has the same output filename as the example target `dump-fixture` in package `astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-engineering)`
  = note: this may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>
  = help: consider changing their names to be unique or compiling them separately
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-journal)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-materials)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-powerplay)
   Compiling astromech-exobiology v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-exobiology)
   Compiling astromech-exploration v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-exploration)
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-voiceeval/crates/astromech-engineering)
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/as
```

