---
created_at: "2026-11-10T05:54:58.675398899+00:00"
id: "astromech-z5oh"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-b2k3"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-k6pl"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-mwh5"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Type roles: every tile, row and card shows one main value at its DESIGN.md size"
updated_at: "2026-11-10T06:04:04.443441037+00:00"
---

## Goal

A commander finds the main value of an element without reading. DESIGN.md gives every element one main value in the largest size (Value XL 28 px on a tile, Value L 18 px on a row), but --fs-xxl is used nowhere and a material tile shows its count at 13 px, the size of its name. Pages name a type role, not a size.

## Done when

- [ ] tokens.css has a class per role in DESIGN.md's Hierarchy table (value-xl, value-l, title, name, label, meta, and the rest), each setting size, weight, colour and tabular figures, using the values astromech-k6pl set.
- [ ] The material tile matches DESIGN.md's Material tile spec (count in Value XL top left, name below, 2 px meter, one corner mark), as in docs/design/glow-up-materials.png, inside the Board layout astromech-mwh5 put the page on.
- [ ] The stat tiles on Exploration, Biology, Session and Mining and the Places home tiles use the roles, each with exactly one Value. Settings > General (put on the Form layout by astromech-mwh5) uses the roles throughout.
- [ ] The font-size part of the page-sizing check (built by astromech-mwh5) is turned on, file by file, for the files that after this item set no font size of their own: at least the material tile's file, the Places home, and Settings > General. A file with other raw sizes left (a table on Exploration, say) stays off the list until its own item converts it.
- [ ] Before-and-after captures of the changed pages at 2560 x 1440 are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Scope

The role classes and the pages named. Pages that get a new screen later take the roles then.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. DESIGN.md Typography and Components. The 10 Nov audit found zero uses of --fs-xxl outside the mockups. Waits on astromech-k6pl (sizes and colours in tokens.css) and astromech-mwh5 (the Materials page's layout and the page-sizing check).
