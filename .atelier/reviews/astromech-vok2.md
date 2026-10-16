---
schema: atelier.review
schema_version: 1
id: astromech-vok2
title: 'R3 [low] crates/astromech-colonisation/src/lib.rs:364-370: the hold that from_journal_dir subtracts comes only from journal events, so drift already deferred in astromech-w73c (CargoTransfer to or from a fleet carrier, count-only Cargo events) now shows in the app as ''N t aboard'' and in the to_buy sent to colonisation_buy. Scenario: the commander moves 700 t of steel from their carrier to the ship; the page still says 0 t aboard and searches for 700 t too many. Cheap fix: take Cargo.json (astromech_journal::parse_cargo, as trade_cargo already does) as the hold''s cargo at the end of the replay when it exists.'
target: astromech-k3wp
identity: review-agent (k3wp review)
commit: f5e62c69c573b451172bf1cd2e16cce3b6519e75
created_at: 2026-10-16T23:10:37.461044275Z
event:
  event: finding
  recommendation: defer
---

R3 [low] crates/astromech-colonisation/src/lib.rs:364-370: the hold that from_journal_dir subtracts comes only from journal events, so drift already deferred in astromech-w73c (CargoTransfer to or from a fleet carrier, count-only Cargo events) now shows in the app as 'N t aboard' and in the to_buy sent to colonisation_buy. Scenario: the commander moves 700 t of steel from their carrier to the ship; the page still says 0 t aboard and searches for 700 t too many. Cheap fix: take Cargo.json (astromech_journal::parse_cargo, as trade_cargo already does) as the hold's cargo at the end of the replay when it exists.
