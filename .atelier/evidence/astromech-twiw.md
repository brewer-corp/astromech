---
created_at: "2026-10-17T00:27:24.598016278+00:00"
id: "astromech-twiw"
evidence_type: "test"
commit: "467ae488f07bd2be3d1861f435d8e3cf0e166a86"
result: "pass"
captured_at: "2026-10-17T00:27:24.298863873+00:00"
command: "cargo test -p astromech-galaxy --lib -- armour_is_found a_pre_engineered_module the_catalogue_is_every spansh_name_variants"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-970a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-970a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R6: armour and pre-engineered searches (base Rail Gun no longer matches an Enduring Feedback seller), catalogue == Spansh field_values/modules both ways (no Basic Discovery Scanner), N1 unknown-symbol name fallback; failed before"
updated_at: "2026-10-17T00:27:24.598263971+00:00"
---

## Summary

R6: armour and pre-engineered searches (base Rail Gun no longer matches an Enduring Feedback seller), catalogue == Spansh field_values/modules both ways (no Basic Discovery Scanner), N1 unknown-symbol name fallback; failed before

## Command

```console
cargo test -p astromech-galaxy --lib -- armour_is_found a_pre_engineered_module the_catalogue_is_every spansh_name_variants
```

Exit status: 0

## Stdout

Bytes: 415
Truncated: no

```text

running 4 tests
test outfitting::tests::the_catalogue_is_every_module_spansh_lists ... ok
test outfitting::tests::spansh_name_variants_fit_by_symbol_or_name ... ok
test stations::tests::armour_is_found_by_its_ship_symbols ... ok
test stations::tests::a_pre_engineered_module_and_its_base_module_are_told_apart ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 34 filtered out; finished in 0.00s
```

## Stderr

Bytes: 161
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-a08adfc363780b79)
```

