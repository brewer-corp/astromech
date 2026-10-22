---
created_at: "2026-10-22T21:30:38.633936421+00:00"
id: "astromech-qmo0"
evidence_type: "test"
commit: "dfcb12fd136d61438891084e3e8672dc52cabd6c"
result: "pass"
captured_at: "2026-10-22T21:30:34.109776853+00:00"
command: "cargo test -p astromech-history --lib ship"
exit_status: "0"
agent_identity: "claude-implementer-fdh3"
target:
  kind: "issue"
  id: "astromech-vett"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-vett"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Trade-in (ShipyardBuy SellShipID 5, ShipyardNew NewShipID 5 of another type) leaves two rows: old sold, new unsold with acquired; ShipRedeemed sets acquired; rebuild gives the same rows; id on another type without a sale closes the old row"
updated_at: "2026-10-22T21:30:38.634397541+00:00"
---

## Summary

Trade-in (ShipyardBuy SellShipID 5, ShipyardNew NewShipID 5 of another type) leaves two rows: old sold, new unsold with acquired; ShipRedeemed sets acquired; rebuild gives the same rows; id on another type without a sale closes the old row

## Command

```console
cargo test -p astromech-history --lib ship
```

Exit status: 0

## Stdout

Bytes: 414
Truncated: no

```text

running 5 tests
test tests::ship_rows_ignore_older_events ... ok
test exploration::tests::landings_count_flown_touchdowns_and_dropships_per_body ... ok
test tests::traded_in_ship_id_reused_by_the_new_ship ... ok
test tests::ship_id_on_another_type_without_a_sale_is_a_new_ship ... ok
test tests::ships_ledger ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.03s
```

## Stderr

Bytes: 222
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running unittests src/lib.rs (target/debug/deps/astromech_history-8b4e2bbed412436b)
```

