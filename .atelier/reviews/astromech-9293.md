---
schema: atelier.review
schema_version: 1
id: astromech-9293
title: 'crates/astromech-colonisation/src/shopping.rs:69-70 (apply_cargo_json): returns early when Cargo.json has no Inventory array. The game writes an empty hold as {Vessel:Ship,Count:0} with no Inventory (parse_cargo at crates/astromech-journal/src/lib.rs:675 and Hold::apply at shopping.rs:30 both treat that as empty). Scenario: the commander moves all cargo to a carrier (the vok2 drift case); Cargo.json becomes Count:0, the replayed hold still says 120 t steel, so ''aboard'' and the tonnage sent to colonisation_buy stay wrong, and the fix is bypassed exactly when the hold is emptied. Fix: for Vessel=Ship, clear the hold when Inventory is absent and Count is 0 (keep it only when Count>0 with no list, as apply does), and add a test for {Vessel:Ship,Count:0} over a non-empty replayed hold.'
target: astromech-k3wp
identity: review-agent (k3wp fix review, sonnet)
commit: 59f832959323c130ff06ac678b9f1b3a7bb80deb
created_at: 2026-10-16T23:18:11.721911460Z
event:
  event: finding
  recommendation: fix_now
---

crates/astromech-colonisation/src/shopping.rs:69-70 (apply_cargo_json): returns early when Cargo.json has no Inventory array. The game writes an empty hold as {Vessel:Ship,Count:0} with no Inventory (parse_cargo at crates/astromech-journal/src/lib.rs:675 and Hold::apply at shopping.rs:30 both treat that as empty). Scenario: the commander moves all cargo to a carrier (the vok2 drift case); Cargo.json becomes Count:0, the replayed hold still says 120 t steel, so 'aboard' and the tonnage sent to colonisation_buy stay wrong, and the fix is bypassed exactly when the hold is emptied. Fix: for Vessel=Ship, clear the hold when Inventory is absent and Count is 0 (keep it only when Count>0 with no list, as apply does), and add a test for {Vessel:Ship,Count:0} over a non-empty replayed hold.
