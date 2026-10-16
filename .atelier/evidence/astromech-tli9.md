---
created_at: "2026-10-16T22:13:59.494267376+00:00"
id: "astromech-tli9"
evidence_type: "test"
commit: "d802250a228aaa0244ebfb10733b355ce22e53e5"
result: "pass"
captured_at: "2026-10-16T22:13:58.910027234+00:00"
command: "cargo test -p astromech-odyssey"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-j4c9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-j4c9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-odyssey tests: ShipLocker+Backpack fixture -> inventory (counts, categories, mission items apart, bartender values); Maverick G1->2 pick shows Carbon Fibre Plating short 2 and G1->5 sums every step; mods with maker costs, fitters, shopping list totals and barter of short assets; engineer unlock status, next step and referral chain; catalogue consistency"
updated_at: "2026-10-16T22:13:59.494763090+00:00"
---

## Summary

astromech-odyssey tests: ShipLocker+Backpack fixture -> inventory (counts, categories, mission items apart, bartender values); Maverick G1->2 pick shows Carbon Fibre Plating short 2 and G1->5 sums every step; mods with maker costs, fitters, shopping list totals and barter of short assets; engineer unlock status, next step and referral chain; catalogue consistency

## Command

```console
cargo test -p astromech-odyssey
```

Exit status: 0

## Stdout

Bytes: 613
Truncated: no

```text

running 7 tests
test tests::missing_files_mean_an_empty_inventory ... ok
test catalog::tests::known_wiki_values ... ok
test catalog::tests::catalogue_is_complete_and_consistent ... ok
test tests::chosen_blueprint_shows_whats_missing ... ok
test tests::modifications_and_the_shopping_list ... ok
test tests::engineer_unlocks_and_referrals ... ok
test tests::shiplocker_fixture_gives_the_inventory ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 315
Truncated: no

```text
   Compiling astromech-odyssey v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-j4c9/crates/astromech-odyssey)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running unittests src/lib.rs (target/debug/deps/astromech_odyssey-a5b98b6d5848dca6)
   Doc-tests astromech_odyssey
```

