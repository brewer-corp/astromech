---
created_at: "2026-10-17T21:31:30.460335641+00:00"
id: "astromech-vett"
issue_type: "bug"
labels:
- "size-m"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-yin0"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ships ledger merges a traded-in ship with the new ship that reuses its id"
updated_at: "2026-10-22T20:41:38.111816974+00:00"
---

## Goal

The history's `ships` ledger keeps one row per ship the commander owned, even when the game gives a new ship the id of the one traded in for it, and a ship still in the fleet is never marked sold.

## Done when

A ledger test with a `ShipyardBuy` that trades in ship 5 (`SellShipID: 5`), then `ShipyardNew` `NewShipID: 5` of another type, leaves two rows: the old type sold, the new type unsold with `acquired` set. A `ShipRedeemed` (`NewShipID`) also sets `acquired`. A rebuild gives the same rows. The yin0 harness (`crates/astromech-core/tests/real_journals.rs`) reports its ships-ledger checks as problems again, not KNOWN (astromech-vett).

## Context

Found replaying real journals (astromech-yin0, harness `crates/astromech-core/tests/real_journals.rs`). The game reuses the traded-in ship's id: in the real logs it happened twice in two weeks (a ship traded in for a new one of another type, which got the same id). The ledger is keyed by `(cmdr, ship_id)` (`crates/astromech-history/src/ledgers.rs`), so the traded-in ships' rows became the new ships and kept the trade-in's `sold`: two ships still in the fleet read as sold, and two types the commander flew have no row. Nothing reads `History::ships` yet, so nobody sees it; fix it before a fleet history page uses it. Likely needs a schema change (a row per ownership, not per id) and a rebuild.
