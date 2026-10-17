---
schema: atelier.review
schema_version: 1
id: astromech-i64s
title: 'medium: Done-when says Ctrl+K then a module name opens its nearby search, but a ship module name (e.g. a fitted Frame Shift Drive) only opens the ship''s build page (app/src/modules/engineering/palette.ts:17-21); only materials reach a nearby search, and the UI check (evidence astromech-bg5d) uses a material. Failure: Ctrl+K, ''frame shift'' lands on Engineering ship page with no lookup. Amend the Done-when / file the module-outfitting follow-up so the closure is honest; the close reason already notes missing systems/stations/commodities but not modules.'
target: astromech-2u1x
identity: review-agent (2u1x review, sonnet)
commit: 75aae4400dc5ad652ba7f629a31f99ad41e2cfbd
created_at: 2026-10-16T22:31:24.033964204Z
event:
  event: finding
  recommendation: fix_now
---

medium: Done-when says Ctrl+K then a module name opens its nearby search, but a ship module name (e.g. a fitted Frame Shift Drive) only opens the ship's build page (app/src/modules/engineering/palette.ts:17-21); only materials reach a nearby search, and the UI check (evidence astromech-bg5d) uses a material. Failure: Ctrl+K, 'frame shift' lands on Engineering ship page with no lookup. Amend the Done-when / file the module-outfitting follow-up so the closure is honest; the close reason already notes missing systems/stations/commodities but not modules.
