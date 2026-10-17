---
schema: atelier.review
schema_version: 1
id: astromech-29ip
title: 'R1 [low] crates/astromech-gamestate/src/mining.rs MiningRun::apply: any LaunchDrone starts a run, including Hatchbreaker, FuelTransfer, Repair, Research and Decontamination limpets (logic from astromech-omi1). Now that the run is shown, a hatchbreaker or repair limpet after undocking makes the Mining page show a run with 0 t and 0 t/h instead of ''No mining yet''. Fix: start only on Prospector/Collection drones (plus ProspectedAsteroid/MiningRefined). Recommendation: DEFER to a follow-up.'
target: astromech-p9r4
identity: review-agent (opus, independent)
commit: 0815c73f5f7e42021269493f59bb4d5720705ef3
created_at: 2026-10-17T04:55:15.942822048Z
event:
  event: finding
  recommendation: defer
---

R1 [low] crates/astromech-gamestate/src/mining.rs MiningRun::apply: any LaunchDrone starts a run, including Hatchbreaker, FuelTransfer, Repair, Research and Decontamination limpets (logic from astromech-omi1). Now that the run is shown, a hatchbreaker or repair limpet after undocking makes the Mining page show a run with 0 t and 0 t/h instead of 'No mining yet'. Fix: start only on Prospector/Collection drones (plus ProspectedAsteroid/MiningRefined). Recommendation: DEFER to a follow-up.
