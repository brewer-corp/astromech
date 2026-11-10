---
created_at: "2026-11-07T03:00:11.378267755+00:00"
id: "astromech-qwvs"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-dsyu"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-k6pl"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-mwh5"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-sixj"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-woqv"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
  - kind: "issue"
    id: "astromech-z5oh"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route page: the timeline"
updated_at: "2026-11-10T06:03:06.734551139+00:00"
---

## Goal

The route shows as the agreed timeline (astromech-ok29's Design), on the Glow-up foundation: star-type icons as nodes, the leg being flown one row a jump with your position, the next stop raised with its copy icon, inhabited stops' second line, Make next on hover. Skip and Back buttons and the 'On your clipboard' line go.

## Done when

- [ ] The leg being flown opens out one row a jump with each star's icon and your row marked whenever the route data lists its jumps (from NavRoute once it is plotted in the galaxy map, or from an exact route's own list). When it lists none it shows its jump count and one line saying the stars appear once the leg is plotted in the galaxy map. A jump whose star is unknown gets the unknown-star icon.
- [ ] A leg ahead shows the stars passed only when the route data lists its jumps (exact routes); otherwise it shows its jump count. The 'nowhere to scoop' warning appears only on a leg where every jump is listed and every one is known not to be scoopable; a leg with a missing jump or an unknown star never claims there is no scoop.
- [ ] The page is built on a layout component and the type roles, sets no width or font size of its own, and passes the page-sizing lint rule; star icons come from Glyph.svelte.
- [ ] The page matches the design in the preview at 2560 x 1440, and at 1100 x 700 with nothing clipped.
- [ ] Copy turns to a check; a failed copy is the red error; Make next works; the stars' tooltips use Tip.
- [ ] Route tests updated; docs/guide's route page describes it.

## Context

Mission astromech-xajc. The data contract is astromech-woqv's: jumps are present for a leg only when a source lists them.
