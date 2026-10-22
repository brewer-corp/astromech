---
created_at: "2026-10-22T20:47:23.855422357+00:00"
id: "astromech-tu34"
evidence_type: "test"
commit: "141d4e4de123567f226682b21e1df86fa00cfd1d"
result: "pass"
captured_at: "2026-10-22T20:47:21.583669014+00:00"
command: "env CARGO_BUILD_JOBS=4 cargo test -p astromech-ship"
exit_status: "0"
agent_identity: "claude-implementer-vlsz"
target:
  kind: "issue"
  id: "astromech-vlsz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-vlsz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fsd.supercharge_multiplier: Mk II size 8 drive gives 6 (any case), plain size 8 SCO and size 6 drives give 4 (astromech-ship the_mk_ii_drive_boosts_six_times, fsd_for_route_plotters)"
updated_at: "2026-10-22T20:47:23.855768973+00:00"
---

## Summary

Fsd.supercharge_multiplier: Mk II size 8 drive gives 6 (any case), plain size 8 SCO and size 6 drives give 4 (astromech-ship the_mk_ii_drive_boosts_six_times, fsd_for_route_plotters)

## Command

```console
env CARGO_BUILD_JOBS=4 cargo test -p astromech-ship
```

Exit status: 0

## Stdout

Bytes: 1909
Truncated: no

```text

running 13 tests
test tests::capacitor_recharge_follows_the_pip_curve ... ok
test tests::unknown_ship_is_an_error ... ok
test module::tests::rail_gun_sustained_counts_its_charge ... ok
test tests::unpowered_drive_has_no_range ... ok
test module::tests::rail_gun_sustained_draw_uses_the_same_cycle_as_sustained_damage ... ok
test module::tests::a_weapon_without_charge_with_a_tiny_reload_keeps_sustained_draw_and_damage_within_burst ... ok
test module::tests::an_engineered_reload_below_the_fire_interval_never_shows_sustained_above_burst ... ok
test module::tests::modifiers_set_the_stats_and_unknown_labels_are_reported ... ok
test module::tests::unknown_grade_is_reported ... ok
test tests::unknown_modules_and_blueprints_are_reported_not_counted ... ok
test module::tests::grade_5_at_roll_0_is_the_grade_4_maximum ... ok
test tests::unknown_hull_is_incomplete_not_an_error ... ok
test module::tests::sustained_never_exceeds_burst_for_any_catalogue_module ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 17 tests
test anaconda_explorer ... ok
test krait_phantom_explorer ... ok
test fsd_for_route_plotters ... ok
test fer_de_lance_modifiers ... ok
test fer_de_lance_sustained_counts_charge ... ok
test krait_mkii_combat ... ok
test anaconda_legacy ... ok
test card_headline_for_the_krait_mkii_combat_build ... ok
test plan_legacy_fsd_long_range ... ok
test fer_de_lance_combat ... ok
test plan_fsd_fast_boot ... ok
test plan_pd_cluster_capacitors ... ok
test the_mk_ii_drive_boosts_six_times ... ok
test python_miner ... ok
test plan_pd_super_conduits ... ok
test plan_thermal_boosters_3 ... ok
test plan_thermal_boosters_4 ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 635
Truncated: no

```text
   Compiling astromech-data v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-vlsz/crates/astromech-data)
   Compiling astromech-engineering v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-vlsz/crates/astromech-engineering)
   Compiling astromech-ship v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-vlsz/crates/astromech-ship)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.99s
     Running unittests src/lib.rs (target/debug/deps/astromech_ship-df27644cecbabd2c)
     Running tests/coriolis.rs (target/debug/deps/coriolis-5e5bd2614de8bd9a)
   Doc-tests astromech_ship
```

