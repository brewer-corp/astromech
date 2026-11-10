---
created_at: "2026-11-10T05:54:59.775498338+00:00"
id: "astromech-dsyu"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
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
title: "Icons for star types, body types and station types"
updated_at: "2026-11-10T05:54:59.775498338+00:00"
---

## Goal

The route timeline, the system tree and the station board mark each star, body and station with an icon a commander learns once, in place of a word per row.

## Done when

- [ ] app/src/lib/ui/Glyph.svelte has, in DESIGN.md's style (16 px grid, 1 px stroke, currentColor, no fill): stars (scoopable, neutron, white dwarf, brown dwarf, black hole, other non-scoopable, and unknown); bodies (rocky, icy, high metal content, water world, Earth-like, ammonia world, gas giant, ringed); and any station kind the existing set lacks.
- [ ] A mockup page (#/mockup/glow-icons) shows every icon with its name at 16 px and at 1.4x, rendered to docs/design/glow-up-icons.png, and each is distinct from the others at 16 px.
- [ ] Each icon has an accessible name where it is used alone; pnpm check and pnpm lint pass.

## Scope

Icons and their sheet. No page uses them yet.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Nothing may be copied from Frontier's icons (DESIGN.md Fixed rules). The Route mockup (astromech-ok29's Design) settled the neutron icon as a pulsar (a core, two beams, a faint ring) and the scoopable star as a plain disc.
