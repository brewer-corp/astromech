---
schema: atelier.item
schema_version: 3
id: astromech-dsyu
title: Icons for star types, body types and station types
status: approved
mission: astromech-xajc
labels:
- size-s
priority: medium
blocked_by:
- astromech-b2k3
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A209be274-1659-4d4c-ac5c-80d4ed74e66c
ready:
  by: conversion
  at: 2026-11-10T05:54:59.775498338Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:54:59.775498338Z
---

## Goal

The route timeline, the system tree and the station board mark each star, body and station with an icon a commander learns once, in place of a word per row.

## Done when

- [x] app/src/lib/ui/Glyph.svelte has, in DESIGN.md's style (16 px grid, 1 px stroke, currentColor, no fill): stars (scoopable, neutron, white dwarf, brown dwarf, black hole, other non-scoopable, and unknown); bodies (rocky, icy, high metal content, water world, Earth-like, ammonia world, gas giant, ringed); and any station kind the existing set lacks.
- [x] A mockup page (#/mockup/glow-icons) shows every icon with its name at 16 px and at 1.4x, rendered to docs/design/glow-up-icons.png, and each is distinct from the others at 16 px.
- [x] Each icon has an accessible name where it is used alone; pnpm check and pnpm lint pass.

## Scope

Icons and their sheet. No page uses them yet.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Nothing may be copied from Frontier's icons (DESIGN.md Fixed rules). The Route mockup (astromech-ok29's Design) settled the neutron icon as a pulsar (a core, two beams, a faint ring) and the scoopable star as a plain disc.

## Report

Rows, trees and the route timeline can now mark a star, body or station with a shape a commander learns once. `app/src/lib/ui/Glyph.svelte` draws them in the existing style (16 px grid, 1 px stroke, currentColor, no fill, square ends). No page uses them yet.

- **Stars** (`star-*`): scoopable (plain disc), neutron (pulsar: core, two beams, faint ring at half opacity), white dwarf, brown dwarf, black hole, other non-scoopable (disc with eight rays) and unknown (dashed circle).
- **Bodies** (`body-*`): rocky, high metal content, metal-rich, icy, rocky ice, water world, Earth-like, ammonia, gas giant, water giant and ringed. The list is the item's plus DESIGN.md's (metal-rich, rocky ice, water giant).
- **Stations**: the existing set lacked `coriolis`, `orbis`, `ocellus` and `asteroid-base`, so those four are new. `starport` and `station` stay because `StationKind.svelte` still maps to them. Pointing StationKind at the new kinds is page work and is out of this item's scope.
- **Accessible names**: a module export `typeNames` maps every type icon to its name ("Neutron star", "Orbis starport"), to pass as `label` when the icon stands alone. The sheet labels every icon. Glyph also takes a `size` prop, so DESIGN.md's 24 px beside a title needs no CSS.
- **Sheet**: `#/mockup/glow-icons` (`app/src/mockups/GlowIcons.svelte`) shows every Glyph, old and new, with its name at 16 px and 1.4x. It is rendered to `docs/design/glow-up-icons.png` by `scripts/mockup-shots.sh glow-icons`. I checked distinctness at 16 px in a 3x capture. The first rocky ice (crater plus a plus sign) was too close to rocky, so it became a polar cap over a crater.
- **DESIGN.md**: the Icons section's star list described main sequence as a disc with eight rays. I updated it to the Route mockup's settled shapes (scoopable is a plain disc, neutron is a pulsar), added other non-scoopable, unknown and ringed, and left the rest unchanged.
- `pnpm check` (0 errors, 0 warnings) and `pnpm lint` pass. Nothing was copied from Frontier's icons; every path is hand-drawn.

