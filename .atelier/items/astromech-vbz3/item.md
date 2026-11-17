---
schema: atelier.item
schema_version: 3
id: astromech-vbz3
title: Engineers search cuts the step left to unlock an engineer at every window size
status: in_review
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Addf2ee2b-9f53-45cd-a676-43c9bd9cce71
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

- [x] At all four checkpoint sizes, the step left reads in full on the row (two lines are fine): `node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only engineers-laser` lists nothing under `clipped`.
- [x] Rows still line up under their group heads (The Aligned Columns Rule), and no action moves more than one column from its row.

## Scope

The Engineers search rows. astromech-m6gw rebuilds the page under this search box; this fix is the rows astromech-7did built.

## Report

The step left now wraps onto a second line in the Engineer column instead of ending in an ellipsis. `app/src/modules/unlocks/Search.svelte` gives `.grp > .step` `white-space: normal` (with `text-wrap: pretty`); the other cells keep their one-line ellipsis. The Tip that carried the full step is gone, since the row now shows it all.

- Audit: `scripts/browser/audit.mjs` and `docs/design/increment2/plan.json` exist only on astromech-8e22's branch (not merged yet), so I ran a copy of both from a scratch folder against `pnpm dev` on this branch, with `--only engineers-laser`. Before: `clipped 1` at 1100x700, 1920x1080, 2560x1440 and 3840x2160, Mel Brandon's step by `ellipsis`. After: `clipped 0` at all four, and overlaps, small, contrast and far stay 0.
- Alignment: in the after captures every row's three groups still start under Reaches, Engineer · your rank and Where, and Add stop and Set destination stay in the actions column. Mel Brandon's row grows by one line ("...past / grade 3.").
- Captures: `docs/design/engineers-step-1100x700-{before,after}.png` and `docs/design/engineers-step-2560x1440-{before,after}.png`.
- `pnpm lint` and `pnpm check` pass. No test: this is a visual change (CONTRIBUTING).

