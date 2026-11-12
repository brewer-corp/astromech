---
schema: atelier.item
schema_version: 3
id: astromech-dsyu
title: Icons for star types, body types and station types
status: building
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

- [ ] app/src/lib/ui/Glyph.svelte has, in DESIGN.md's style (16 px grid, 1 px stroke, currentColor, no fill): stars (scoopable, neutron, white dwarf, brown dwarf, black hole, other non-scoopable, and unknown); bodies (rocky, icy, high metal content, water world, Earth-like, ammonia world, gas giant, ringed); and any station kind the existing set lacks.
- [ ] A mockup page (#/mockup/glow-icons) shows every icon with its name at 16 px and at 1.4x, rendered to docs/design/glow-up-icons.png, and each is distinct from the others at 16 px.
- [ ] Each icon has an accessible name where it is used alone; pnpm check and pnpm lint pass.

## Scope

Icons and their sheet. No page uses them yet.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Nothing may be copied from Frontier's icons (DESIGN.md Fixed rules). The Route mockup (astromech-ok29's Design) settled the neutron icon as a pulsar (a core, two beams, a faint ring) and the scoopable star as a plain disc.
