---
schema: atelier.item
schema_version: 3
id: astromech-qwvs
title: 'Route page: the timeline'
status: building
mission: astromech-xajc
labels:
- size-m
priority: high
blocked_by:
- astromech-dsyu
- astromech-k6pl
- astromech-mwh5
- astromech-sixj
- astromech-woqv
- astromech-z5oh
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A5e78e1b5-3f1e-4b4d-b3ed-888d41e195f4
- 01a1cf5f-ca4c-7342-a8e0-fddadad8e1af
ready:
  by: conversion
  at: 2026-11-10T06:03:06.734551139Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:11.378267755Z
---

## Goal

The route shows as the agreed timeline (astromech-ok29's Design), on the Glow-up foundation: star-type icons as nodes, the leg being flown one row a jump with your position, the next stop raised with its copy icon, inhabited stops' second line, Make next on hover. Skip and Back buttons and the 'On your clipboard' line go.

## Done when

- [ ] The leg being flown opens out one row a jump with each star's icon and your row marked whenever the route data lists its jumps (from NavRoute once it is plotted in the galaxy map, or from an exact route's own list). When it lists none it shows its jump count and one line saying the stars appear once the leg is plotted in the galaxy map. A jump whose star is unknown gets the unknown-star icon.
- [ ] A leg ahead shows the stars passed only when the route data lists its jumps (exact routes); otherwise it shows its jump count. The 'nowhere to scoop' warning appears only on a leg where every jump is listed and every one is known not to be scoopable; a leg with a missing jump or an unknown star never claims there is no scoop.
- [ ] The page is built on a layout component and the type roles, sets no width or font size of its own, and its files pass both parts of the page-sizing check; star icons come from Glyph.svelte.
- [ ] The page matches the design in the preview at 2560 x 1440, and at 1100 x 700 with nothing clipped.
- [ ] Copy turns to a check; a failed copy is the red error; Make next works; the stars' tooltips use Tip.
- [ ] Route tests updated; docs/guide's route page describes it.

## Context

Mission astromech-xajc. The data contract is astromech-woqv's: jumps are present for a leg only when a source lists them.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-ok29 (Route page redesign: the title is the plotter, the route a timeline)
