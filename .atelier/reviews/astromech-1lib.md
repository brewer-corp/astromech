---
schema: atelier.review
schema_version: 1
id: astromech-1lib
title: finding on astromech-sllo @1fd71219
target: astromech-sllo
identity: reviewer-fable-sllo
commit: 1fd712195d1b0ebd90f65ccdd3d7f695e3f16e83
created_at: 2026-11-05T18:50:33.904395780Z
event:
  event: finding
  recommendation: fix_now
---

low: app/src/modules/materials/View.svelte:243 the second .find rule (added by kwoi for the <p class="find"> sites line at :168) also styles the div.find pane at :104: padding 0 8px, margin-top 24px, Orange Mid, 13px. This branch removed .query's own top padding, so the new Kind choice row's vertical position now depends on that accidental margin, and its left edge is 46 px while On foot, Engineers, Ships and Controls rows sit at the 38 px page margin (measured in Chrome at 1280 px); tile names and counts inherit Orange Mid from the same rule. Fix now (decision Q5, and Q4 for the control rows): rename the sites line's class (e.g. .sites) and give .query the same padding: var(--sp-6) 0 0 the unlocks and odyssey pages use; astromech-0132 then restyles the tiles from a clean base.
