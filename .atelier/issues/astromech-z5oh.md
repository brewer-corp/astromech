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
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Type roles: every tile, row and card shows one main value at its DESIGN.md size"
updated_at: "2026-11-10T05:54:58.675398899+00:00"
---

## Goal

A commander finds the main value of an element without reading. DESIGN.md gives every element one main value in the largest size (Value XL 28 px on a tile, Value L 18 px on a row), but --fs-xxl is used nowhere and a material tile shows its count at 13 px, the size of its name. Pages name a type role, not a size.

## Done when

- [ ] tokens.css has a class per role in DESIGN.md's Hierarchy table (value-xl, value-l, title, name, label, meta, and the rest), each setting size, weight, colour and tabular figures.
- [ ] The material tile matches DESIGN.md's Material tile spec (count in Value XL top left, name below, 2 px meter, one corner mark), as in docs/design/glow-up-materials.png.
- [ ] The stat tiles on Exploration, Biology, Session and Mining and the Places home tiles use the roles, each with exactly one Value.
- [ ] A rule in app/eslint-rules fails on a raw font-size in a file under app/src/modules/, enabled for the modules converted here, with its own test.
- [ ] Before-and-after captures of the changed pages at 2560 x 1440 are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Scope

The role classes, the lint rule, and the pages named. Pages that get a new screen later take the roles then.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. DESIGN.md Typography and Components. The 10 Nov audit found zero uses of --fs-xxl outside the mockups.
