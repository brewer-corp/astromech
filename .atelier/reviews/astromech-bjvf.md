---
schema: atelier.review
schema_version: 1
id: astromech-bjvf
title: GameState keeps ShipID; a Loadout for a different ship clears last_jump, the same ship's Loadout keeps it. Test another_ship_starts_without_a_fuel_estimate (fails without the fix).
target: astromech-77a1
identity: orchestrator (Claude, not a code author)
commit: 9f945111d1193b4d9ec0b67802eba8dcc70b0c41
created_at: 2026-10-17T02:12:28.355960882Z
event:
  event: disposition
  finding: astromech-davf
  decision: fix_now
  supersedes: null
  resolution_commit: 9f945111d1193b4d9ec0b67802eba8dcc70b0c41
  follow_up: null
---

GameState keeps ShipID; a Loadout for a different ship clears last_jump, the same ship's Loadout keeps it. Test another_ship_starts_without_a_fuel_estimate (fails without the fix).
