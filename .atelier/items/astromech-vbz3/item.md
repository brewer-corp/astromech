---
schema: atelier.item
schema_version: 3
id: astromech-vbz3
title: Engineers search cuts the step left to unlock an engineer at every window size
status: ready
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T10:14:46.678176257Z
trivial: null
caused_by: astromech-7did
created_at: 2026-11-17T10:08:06.558283739Z
---

## Goal

The Engineers search shows the whole step left to unlock an engineer, since it is what the commander has to do next. Found by astromech-8e22 on 17 Nov 2026: with "laser" searched, Mel Brandon's step reads "Introduction from Elvira Martuuk: raise your rank with them past ..." at every size from 1100 x 700 to 3840 x 2160. The `.grp` children in `app/src/modules/unlocks/Search.svelte` are `nowrap` with an ellipsis, and the full text is only in a Tip. Captures: docs/design/increment2/engineers-laser-*.png.

## Done when

- [ ] At all four checkpoint sizes, the step left reads in full on the row (two lines are fine): `node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only engineers-laser` lists nothing under `clipped`.
- [ ] Rows still line up under their group heads (The Aligned Columns Rule), and no action moves more than one column from its row.

## Scope

The Engineers search rows. astromech-m6gw rebuilds the page under this search box; this fix is the rows astromech-7did built.
