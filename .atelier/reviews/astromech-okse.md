---
schema: atelier.review
schema_version: 1
id: astromech-okse
title: finding on astromech-51ue @0b6806b9
target: astromech-51ue
identity: reviewer-sonnet-f7i0
commit: 0b6806b99ee217651e9f848144dc2f608e9b5db2
created_at: 2026-11-05T09:13:09.075228247Z
event:
  event: finding
  recommendation: fix_now
---

medium: astromech-f7i0 crates/astromech-market/src/mining.rs:70 (BODIES=50) and app/src/lib/ui/SearchKit.svelte summary: the finder says 'Within 100 ly of Sol' and ranks 'most hotspots first', but it asks Spansh for only the nearest 50 bodies in the radius and ranks those. The recorded Painite answer (tests/fixtures/spansh-hotspots-painite-sol.json, 'any': count 10000, 30 bodies reach only 28.4 ly) shows 50 bodies reach about 35 ly, so near Sol the 100 ly radius is mostly unsearched and the top-count rings (the main value, the ranking) are missed; the 10-row cut (LIMIT in main.rs/lib/hotspots.ts) has no 'N more' either, and ring type is filtered after the 50. Only guide places.md 'Limits' says so; the window and the summary contradict it. Fix: carry a capped flag on Hotspots (bodies returned == BODIES) and let the summary name the window (e.g. 'nearest 50 bodies, 12 rings of 30') instead of the radius when capped, or page Spansh until the radius is covered; add a recorded test for a capped answer.
