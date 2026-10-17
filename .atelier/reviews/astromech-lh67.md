---
schema: atelier.review
schema_version: 1
id: astromech-lh67
title: 'R7: server-side re-resolution is on main as claimed (search_origin calls Filters::for_ship on every galaxy/stations/sites/colony/trade search), but the filter box label ''Ship''s (X)'' (app/src/lib/ui/SearchFilters.svelte:60) still shows the pad from when the lookup''s filters loaded, so after a ship change the UI shows the old pad while searching with the new one. The done-when test (''a ship change without re-selecting the lookup'') was not written; only the Filters::for_ship rule is tested, and nothing fails if search_origin stops calling it.'
target: astromech-n1ij
identity: review-agent (opus, independent)
commit: df2c4fb438a4f201a3a05fc02770c5536db43657
created_at: 2026-10-17T05:05:44.208576232Z
event:
  event: finding
  recommendation: defer
---

R7: server-side re-resolution is on main as claimed (search_origin calls Filters::for_ship on every galaxy/stations/sites/colony/trade search), but the filter box label 'Ship's (X)' (app/src/lib/ui/SearchFilters.svelte:60) still shows the pad from when the lookup's filters loaded, so after a ship change the UI shows the old pad while searching with the new one. The done-when test ('a ship change without re-selecting the lookup') was not written; only the Filters::for_ship rule is tested, and nothing fails if search_origin stops calling it.
