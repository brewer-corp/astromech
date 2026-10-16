---
created_at: "2026-10-16T19:06:48.687213286+00:00"
id: "astromech-8ym7"
evidence_type: "test"
commit: "1ba680f364e56a67db22b6af8aed7c9ab6644477"
result: "pass"
captured_at: "2026-10-16T19:06:45.057834539+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-engineering && cd app && pnpm test'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-9mb9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-9mb9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-engineering && cd app && pnpm test'"
updated_at: "2026-10-16T19:06:48.687859291+00:00"
---

## Summary

bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-engineering && cd app && pnpm test'

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3; cargo test -p astromech-engineering && cd app && pnpm test'
```

Exit status: 0

## Stdout

Bytes: 1362
Truncated: no

```text

running 15 tests
test tests::farm_route_trades_down_from_anchors ... ok
test links::tests::coriolis_round_trips_the_loadout ... ok
test tests::trader_rates ... ok
test links::tests::edsy_round_trips_the_loadout ... ok
test tests::trades_and_farm_respect_storage_caps ... ok
test links::tests::urls_snapshot ... ok
test tests::single_grade_blueprints_and_vehicles ... ok
test tests::catalogue_materials_resolve ... ok
test tests::other_blueprint_restarts_and_bad_targets ... ok
test tests::crafts_update_the_fleet_and_shrink_the_trip ... ok
test tests::fleet_from_history ... ok
test tests::grades_remaining_and_shopping_list ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok
test tests::experimental_never_before_its_blueprint ... ok
test tests::trip_in_person_stops_and_remote_note ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-task-astromech-9mb9/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-task-astromech-9mb9/app


 Test Files  1 passed (1)
      Tests  4 passed (4)
   Start at  15:06:48
   Duration  201ms (import 45%, transform 23%, tests 17%, worker 15%, environment 1%)
```

## Stderr

Bytes: 331
Truncated: no

```text
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-9mb9/crates/astromech-engineering)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-d675e23c1082a93d)
   Doc-tests astromech_engineering
```

