---
schema: atelier.review
schema_version: 1
id: astromech-f1fo
title: finding on astromech-rz0f @2828a4c0
target: astromech-rz0f
identity: reviewer-fable-rz0f
commit: 2828a4c0e6b4301e193a1587c21be434e740b88f
created_at: 2026-11-05T07:28:23.013098360Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/route.ts:96 followedNote's 'Replaced <name>' names view.parked, which can be a route parked earlier by a galaxy-map detour rather than the one this action replaced: with a trip parked under a galaxy-map route, Set destination drops the game route (not parked, by design) and the note says 'Replaced <trip>'. Resume still restores the right route, so acceptable for now (decision below, question 5). Later: have the core report what this call parked (a field in View or a return from Desk::set), or word it 'Paused: <name>'.
