---
schema: atelier.review
schema_version: 2
id: astromech-ttf7
title: finding on astromech-bp23 @10994559
target: astromech-bp23
identity: claude-reviewer-96dv
commit: 109945598112dc967a6781b2fcb4cf70ae23aae2
created_at: 2026-11-07T10:45:19.871380136Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/preview/fixtures/plot.json:103 (96dv) The local neutron route's 150 per-stop 'jumps' (1 per leg, 2 on every fifth, chosen to sum to the recorded 180) are invented, not recorded; decision astromech-1zhg says fixtures are recorded raw inputs or dumps, never invented replies, and CONTRIBUTING's new line says to take a missing field from the recorded input or its dump. The preview's route strip and Plot result after a preview neutron download now show made-up per-stop counts as if the plotter produced them. Fix: re-record the local route with the real local plotter (astromech_galaxy::plot::plot_route on a neutron file built by scripts/gen-neutron.py, on boat if heavy, with the fixture's ship), or have the plan owner/Sumeet record an explicit exception in 96dv with a filed issue to re-record. (The other plausible fills, stations filters max_distance_ly/min_volume null, exobiology analysed false, voice live_from null, follow deterministically from the recorded inputs and the core's code: no action.)
