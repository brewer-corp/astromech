---
created_at: "2026-11-07T02:59:43.905221193+00:00"
id: "astromech-ok29"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-irrv"
  - kind: "issue"
    id: "astromech-qwvs"
  - kind: "issue"
    id: "astromech-woqv"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route page redesign: the title is the plotter, the route a timeline"
updated_at: "2026-11-10T06:03:07.099931163+00:00"
---

## Goal

The Galaxy > Route page shows where you are and what's next at a glance, and is where you plot. Agreed with Sumeet on 6 Nov 2026 from the artifact mockup https://claude.ai/artifact/JduR92Bxotyi7hd6Jg8nN7 (version 9); built in the real page against the browser preview, shared with scripts/preview-publish. It is the first screen built on the Glow-up foundation (mission astromech-xajc).

## Done when

- [ ] Every child is done and the page matches the Design below in the preview at 2560 x 1440.
- [ ] The page uses a layout component, the type roles, Tip and Glyph's star icons, and sets no page-level width or font size (both parts of the page-sizing check are on for the Route page's own files, View.svelte and the components the page is built from, and pass; The frame's own parts that live in the route module (Strip.svelte, the route strip, and PlotBar.svelte, the header's Find or plot) are not pages and are exempt).
- [ ] The page holds up in the preview at 1100 x 700 and 2560 x 1440 with the height scale on, and the route tests pass.

## Design

- The title is the plotter: `Khepri to [destination]`, the destination editable in place; Plotter (Neutron, Exact) and Worked out (On this PC, By Spansh) dropdowns, the neutron star file's status, download and check in the second; one Replot from here; a line when the ship's range differs from the range it was plotted at. No side panel, no import box: Import, Open on Spansh and Clear in a ⋯ menu; the galaxy-map help behind a ?.
- The route is a timeline whose nodes are the stars' type icons (scoopable, neutron, white dwarf, brown dwarf). The leg you are on opens out one row a jump, your row highlighted with a map pin, once the game's NavRoute lists its jumps. No distance columns.
- Changed from the 6 Nov mockup (planning review, 10 Nov 2026): the mockup drew the stars passed on every leg. The neutron plotters give only a jump count for a leg, so a leg ahead shows its jump count, and shows the stars passed only where the data lists them (exact routes, and the flown leg from NavRoute). The 'nowhere to scoop' warning appears only where every jump of the leg is listed and known not to be scoopable.
- The next stop is the one raised row: larger name, its copy icon beside it, "N jumps away". Other stops: copy icon and Make next on hover. Skip and Back buttons go (voice keeps them).
- Inhabited systems get a second line: allegiance and government, security, population, controlling faction and its state.
- No "On your clipboard" line: the copy icon turns to a check; a failed copy is the red error.
