---
schema: atelier.item
schema_version: 3
id: astromech-z5oh
title: 'Type roles: every tile, row and card shows one main value at its DESIGN.md size'
status: in_review
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-b2k3
- astromech-k6pl
- astromech-mwh5
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A419fed58-3752-414c-b007-1be2aa87edff
ready:
  by: conversion
  at: 2026-11-10T06:04:04.443441037Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:54:58.675398899Z
---

## Goal

A commander finds the main value of an element without reading. DESIGN.md gives every element one main value in the largest size (Value XL 28 px on a tile, Value L 18 px on a row), but --fs-xxl is used nowhere and a material tile shows its count at 13 px, the size of its name. Pages name a type role, not a size.

## Done when

- [x] tokens.css has a class per role in DESIGN.md's Hierarchy table (value-xl, value-l, title, name, label, meta, and the rest), each setting size, weight, colour and tabular figures, using the values astromech-k6pl set.
- [x] The material tile matches DESIGN.md's Material tile spec (count in Value XL top left, name below, 2 px meter, one corner mark), as in docs/design/glow-up-materials.png, inside the Board layout astromech-mwh5 put the page on.
- [x] The stat tiles on Exploration, Biology, Session and Mining and the Places home tiles use the roles, each with exactly one Value. Settings > General (put on the Form layout by astromech-mwh5) uses the roles throughout.
- [x] The font-size part of the page-sizing check (built by astromech-mwh5) is turned on, file by file, for the files that after this item set no font size of their own: at least the material tile's file, the Places home, and Settings > General. A file with other raw sizes left (a table on Exploration, say) stays off the list until its own item converts it.
- [x] Before-and-after captures of the changed pages at 2560 x 1440 are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Scope

The role classes and the pages named. Pages that get a new screen later take the roles then.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. DESIGN.md Typography and Components. The 10 Nov audit found zero uses of --fs-xxl outside the mockups. Waits on astromech-k6pl (sizes and colours in tokens.css) and astromech-mwh5 (the Materials page's layout and the page-sizing check).

## Report

Every tile now shows its one main value at DESIGN.md's size, and the pages named name a type
role instead of a font size.

- **Role classes.** `app/src/design/tokens.css` has `.type-wordmark`, `.type-title`,
  `.type-value-xl`, `.type-value-l`, `.type-nav`, `.type-name`, `.type-body`, `.type-figures`,
  `.type-label`, `.type-meta` and `.type-action`, each with the Hierarchy table's size, weight,
  line height, case and colour from the existing `--fs-*` and colour tokens; Section stays the
  existing `.section`. They are prefixed `type-` because 72 files already use `.name`, 34 `.label`
  and 25 `.title` for their own parts, and unprefixed global classes would have restyled them.
  `.type-unit` is the "unit after its number" rule. DESIGN.md's Token map lists them.
- **Material tile** (`modules/materials/View.svelte`): the count in Value XL top left, the name
  below in Label size and Orange, the 2 px track, the grade's cap once in the column head
  ("Grade 1 · max 300"), inside the Board astromech-mwh5 built. The core sends no plan needs yet,
  so the raw, manufactured and encoded tiles have no corner mark; Other's tiles, whose caps
  differ, use the one Meta corner mark for their cap. Each tile's accessible name is
  "Iron, 22 of 300", so the cap is still said. The side panel's title is Title, its held count
  Value XL in white with "of 300 held", as the mockup. Compare
  `docs/design/type-roles-materials-selected-after.png` with `glow-up-materials.png` on the
  `mockups/glow-up` branch (it is not on main).
- **Stat tiles** on Exploration, Biology, Session and Mining are one new `lib/ui/Stat.svelte`: a
  Label, one Value XL, its unit in Label after a real space (so "9 t" reads as one phrase), and an
  optional Label line. `lib/exploration.ts` gains `creditParts()` so "33.3 M cr" splits into the
  figure and its unit. Biology's Species and "Far enough" are words, shown in the Value role too,
  since each is that tile's one value.
- **Places home** and **Settings > General** (with its Callouts and Prospector thresholds parts)
  use the roles throughout: row labels Name in Options Grey, hints Label in Prose Grey, paths
  Label. Settings values in cells keep the page's base 15/500 text: DESIGN.md's component table
  gives inputs the Body role (400), but its Typography section says orange at 400 looked thin, so
  I left them unchanged rather than pick between the two.
- **Font-size check** (`app/eslint.config.js`) is on for `materials/View`, `places/Home`,
  `settings/View`, `settings/{Callouts,ProspectorThresholds}`, and `{session,mining}/View`.
  Exploration and Biology stay off: their tables still set sizes.
- **Tests.** The stat tiles split value and unit into two elements, so Session, Mining and
  Exploration tests read a tile's value through a small `src/test/stat.ts` helper; two App tests
  query `.type-value-l` in place of `.v`. `pageSizing.test.js` checks the font-size part where it
  is on. The guide's Materials page describes the new tile.
- **Captures** at 2560 x 1440: `docs/design/type-roles-{materials,materials-selected,places-home,
  settings,exploration,biology,session,mining}-{before,after}.png`.
- pnpm check, pnpm lint and pnpm test (612) pass; `scripts/check-guide.py` passes.


### Responses

1-i1na.1: fixed in f060e9d2 (both warnings are now `hint type-label warn-text`)
1-i1na.2: fixed in f060e9d2 (recaptured from the submitted source; no overlay)
