---
schema: atelier.review
schema_version: 1
id: astromech-davf
title: 'crates/astromech-gamestate/src/lib.rs:162-176: last_jump (the fuel-estimate baseline) is not reset when the Loadout is for a different ship. After a ShipyardSwap, the first arrival and any route plotted before the first jump use the previous ship''s fuel per light year, which can be several times off (different mass and optimal mass) in either direction. Fix: clear last_jump when the Loadout''s ship id changes.'
target: astromech-77a1
identity: review-agent (77a1 review)
commit: 94747f219dd1884a56d2a091af42f68fc031ff7e
created_at: 2026-10-17T02:05:40.271237819Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-gamestate/src/lib.rs:162-176: last_jump (the fuel-estimate baseline) is not reset when the Loadout is for a different ship. After a ShipyardSwap, the first arrival and any route plotted before the first jump use the previous ship's fuel per light year, which can be several times off (different mass and optimal mass) in either direction. Fix: clear last_jump when the Loadout's ship id changes.
