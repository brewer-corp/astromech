---
schema: atelier.review
schema_version: 1
id: astromech-ojnm
title: apply_cargo_json clears the hold for a ship Cargo.json with Count 0 and no Inventory, keeps it for a count with no list (as Hold::apply); test adds {Vessel:Ship,Count:0} over a 120 t replayed hold and fails on the old early return
target: astromech-k3wp
identity: orchestrator (Claude, not a code author)
commit: f219f91bfddf01b3ccbe1d7a4046ff179bae2a31
created_at: 2026-10-16T23:20:05.712387187Z
event:
  event: disposition
  finding: astromech-9293
  decision: fix_now
  supersedes: null
  resolution_commit: f219f91bfddf01b3ccbe1d7a4046ff179bae2a31
  follow_up: null
---

apply_cargo_json clears the hold for a ship Cargo.json with Count 0 and no Inventory, keeps it for a count with no list (as Hold::apply); test adds {Vessel:Ship,Count:0} over a 120 t replayed hold and fails on the old early return
