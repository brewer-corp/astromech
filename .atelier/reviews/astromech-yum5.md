---
schema: atelier.review
schema_version: 1
id: astromech-yum5
title: 'crates/astromech-gamestate/src/lib.rs:231-232 (used by fuel.rs:47-50 hop_fuel and route.rs fuel_line): last_jump is overwritten by every FSDJump, including supercharged ones. On a neutron or white-dwarf boost (or a synthesis FSD injection), JumpDist is the boosted distance (up to 4x) while FuelUsed is the fuel for the unboosted jump, so f0*(d/d0)^p underestimates later hops by about 4^2.45 ~ 30x. Failure: on a neutron-highway route (the case the route rule names), after the first boosted jump fuel_range and the route fuel line never warn even when the tank is nearly dry. The journal marks these jumps (FSDJump BoostUsed, already listed in astromech-eddn/src/journal.rs). Fix: parse BoostUsed and keep the previous last_jump when it is present, with a replay test (a boosted jump followed by a short tank still warns).'
target: astromech-77a1
identity: review-agent (77a1 review)
commit: 94747f219dd1884a56d2a091af42f68fc031ff7e
created_at: 2026-10-17T02:05:40.190495765Z
event:
  event: finding
  recommendation: fix_now
---

crates/astromech-gamestate/src/lib.rs:231-232 (used by fuel.rs:47-50 hop_fuel and route.rs fuel_line): last_jump is overwritten by every FSDJump, including supercharged ones. On a neutron or white-dwarf boost (or a synthesis FSD injection), JumpDist is the boosted distance (up to 4x) while FuelUsed is the fuel for the unboosted jump, so f0*(d/d0)^p underestimates later hops by about 4^2.45 ~ 30x. Failure: on a neutron-highway route (the case the route rule names), after the first boosted jump fuel_range and the route fuel line never warn even when the tank is nearly dry. The journal marks these jumps (FSDJump BoostUsed, already listed in astromech-eddn/src/journal.rs). Fix: parse BoostUsed and keep the previous last_jump when it is present, with a replay test (a boosted jump followed by a short tank still warns).
