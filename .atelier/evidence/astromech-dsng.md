---
created_at: "2026-10-17T00:27:31.443905791+00:00"
id: "astromech-dsng"
evidence_type: "test"
commit: "68d91e885079728ef2b8dc52c70fecf15cdb79af"
result: "pass"
captured_at: "2026-10-17T00:27:25.314064583+00:00"
command: "cargo test -p astromech-market --test anywhere"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-d1lo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-d1lo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Recorded-response test (Ardent galaxy-wide imports/exports captured 17 Oct 2026, trimmed): painite sell anywhere lists markets highest price first with price, demand and updatedAt, measured from Sol with no radius (582 ly carrier first); 30-day default and fleetCarriers=false asked of Ardent; pad, Odyssey and arrival applied locally; tritium buy cheapest first, nearer first on ties, carriers excluded via Ardent. A mutation applying the nearby 100 ly default fails all 3."
updated_at: "2026-10-17T00:27:31.444304816+00:00"
---

## Summary

Recorded-response test (Ardent galaxy-wide imports/exports captured 17 Oct 2026, trimmed): painite sell anywhere lists markets highest price first with price, demand and updatedAt, measured from Sol with no radius (582 ly carrier first); 30-day default and fleetCarriers=false asked of Ardent; pad, Odyssey and arrival applied locally; tritium buy cheapest first, nearer first on ties, carriers excluded via Ardent. A mutation applying the nearby 100 ly default fails all 3.

## Command

```console
cargo test -p astromech-market --test anywhere
```

Exit status: 0

## Stdout

Bytes: 334
Truncated: no

```text

running 3 tests
test sell_anywhere_lists_the_best_prices_in_order_with_data_age ... ok
test buy_anywhere_lists_the_cheapest_first_and_asks_ardent_to_drop_carriers ... ok
test sell_anywhere_applies_the_data_age_and_station_filters ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 2455
Truncated: no

```text
   Compiling smallvec v1.16.3
   Compiling libc v0.2.190
   Compiling serde_core v1.0.229
   Compiling syn v3.0.7
   Compiling num-traits v0.2.19
   Compiling serde_json v1.0.152
   Compiling serde v1.0.229
   Compiling memchr v2.8.3
   Compiling crc32fast v1.5.2
   Compiling hashbrown v0.17.1
   Compiling synstructure v0.14.0
   Compiling zerovec-derive v0.11.6
   Compiling displaydoc v0.2.7
   Compiling zerofrom-derive v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling getrandom v0.2.17
   Compiling thiserror-impl v2.0.21
   Compiling zerofrom v0.1.8
   Compiling serde_derive v1.0.229
   Compiling ring v0.17.14
   Compiling yoke v0.8.3
   Compiling zerotrie v0.2.5
   Compiling simd-adler32 v0.3.10
   Compiling zerovec v0.11.8
   Compiling percent-encoding v2.3.2
   Compiling chrono v0.4.45
   Compiling form_urlencoded v1.2.2
   Compiling tinystr v0.8.4
   Compiling potential_utf v0.1.6
   Compiling icu_collections v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling miniz_oxide v0.9.1
   Compiling rustls-webpki v0.103.15
   Compiling thiserror v2.0.21
   Compiling hashlink v0.12.2
   Compiling icu_provider v2.3.1
   Compiling bitflags v2.13.2
   Compiling log v0.4.34
   Compiling flate2 v1.1.10
   Compiling icu_properties v2.3.0
   Compiling icu_normalizer v2.3.0
   Compiling rustls v0.23.46
   Compiling rusqlite v0.40.2
   Compiling idna_adapter v1.2.2
   Compiling idna v1.1.0
   Compiling url v2.5.8
   Compiling ureq v2.12.1
   Compiling astromech-store v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-store)
   Compiling astromech-journal v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-journal)
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-data)
   Compiling astromech-powerplay v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-powerplay)
   Compiling astromech-materials v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-materials)
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-d1lo/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.95s
     Running tests/anywhere.rs (target/debug/deps/anywhere-2972c20cbf98a0fd)
```

