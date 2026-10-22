---
created_at: "2026-10-22T23:49:40.499285432+00:00"
id: "astromech-aqfc"
evidence_type: "validation"
commit: "327c059301ee2d1a145df8472e7f626f461ac933"
result: "blocked"
captured_at: "2026-10-22T23:49:40.499221624+00:00"
agent_identity: "validator-e8ux"
target:
  kind: "issue"
  id: "astromech-e8ux"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e8ux"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S7 Reactions: blocked on the play session; journal replay part passes. At 327c059 (fresh workspace run): astromech-announce replay tests pass: prospector 7, scan 4, arrival 16, docking 4, fuel 10, route 9, replay 8 (cooldown, silent catch-up), plus powerplay_arrival 7, signals 2, hge 6. Real-journal harness on a copy (0 problems): replay of 35955 lines speaks 0 callouts; with every line treated as live, prospector 5, scan 43, arrival 98, docking 89 (pad position 'at three, back', denial reasons) fire on real data without panics; fuel and route never fired because the harness feeds no NavRoute.json (fixtures cover them). HUD rule by inspection: arrival never says star class, security, allegiance or the name; fuel does not repeat the low-fuel warning or scoop progress (and its test asserts silence at a scoopable star); signals skips stations/carriers and HGE. Blocked: 'shown by a play session' and 'nothing already prominent on the HUD is spoken' in game need astromech-505u (arrival, docking, route callouts) and astromech-6920 (mining with prospector and scan callouts), both needs-human."
updated_at: "2026-10-22T23:49:40.499581995+00:00"
---

S7 Reactions: blocked on the play session; journal replay part passes. At 327c059 (fresh workspace run): astromech-announce replay tests pass: prospector 7, scan 4, arrival 16, docking 4, fuel 10, route 9, replay 8 (cooldown, silent catch-up), plus powerplay_arrival 7, signals 2, hge 6. Real-journal harness on a copy (0 problems): replay of 35955 lines speaks 0 callouts; with every line treated as live, prospector 5, scan 43, arrival 98, docking 89 (pad position 'at three, back', denial reasons) fire on real data without panics; fuel and route never fired because the harness feeds no NavRoute.json (fixtures cover them). HUD rule by inspection: arrival never says star class, security, allegiance or the name; fuel does not repeat the low-fuel warning or scoop progress (and its test asserts silence at a scoopable star); signals skips stations/carriers and HGE. Blocked: 'shown by a play session' and 'nothing already prominent on the HUD is spoken' in game need astromech-505u (arrival, docking, route callouts) and astromech-6920 (mining with prospector and scan callouts), both needs-human.
