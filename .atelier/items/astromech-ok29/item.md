---
schema: atelier.item
schema_version: 3
id: astromech-ok29
title: 'Route page redesign: the title is the plotter, the route a timeline'
status: approved
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3Ab936fcf5-741d-4f09-9f7a-efe41cb63279
ready:
  by: conversion
  at: 2026-11-10T06:03:07.099931163Z
trivial: null
caused_by: null
created_at: 2026-11-07T02:59:43.905221193Z
---

## Goal

The Galaxy > Route page shows where you are and what's next at a glance, and is where you plot. Agreed with Sumeet on 6 Nov 2026 from the artifact mockup https://claude.ai/artifact/JduR92Bxotyi7hd6Jg8nN7 (version 9); built in the real page against the browser preview, shared with scripts/preview-publish. It is the first screen built on the Glow-up foundation (mission astromech-xajc).

## Done when

- [x] Every child is done and the page matches the Design below in the preview at 2560 x 1440.
- [x] The page uses a layout component, the type roles, Tip and Glyph's star icons, and sets no page-level width or font size (both parts of the page-sizing check are on for the Route page's own files, View.svelte and the components the page is built from, and pass; The frame's own parts that live in the route module (Strip.svelte, the route strip, and PlotBar.svelte, the header's Find or plot) are not pages and are exempt).
- [x] The page holds up in the preview at 1100 x 700 and 2560 x 1440 with the height scale on, and the route tests pass.

## Design

- The title is the plotter: `Khepri to [destination]`, the destination editable in place; Plotter (Neutron, Exact) and Worked out (On this PC, By Spansh) dropdowns, the neutron star file's status, download and check in the second; one Replot from here; a line when the ship's range differs from the range it was plotted at. No side panel, no import box: Import, Open on Spansh and Clear in a ⋯ menu; the galaxy-map help behind a ?.
- The route is a timeline whose nodes are the stars' type icons (scoopable, neutron, white dwarf, brown dwarf). The leg you are on opens out one row a jump, your row highlighted with a map pin, once the game's NavRoute lists its jumps. No distance columns.
- Changed from the 6 Nov mockup (planning review, 10 Nov 2026): the mockup drew the stars passed on every leg. The neutron plotters give only a jump count for a leg, so a leg ahead shows its jump count, and shows the stars passed only where the data lists them (exact routes, and the flown leg from NavRoute). The 'nowhere to scoop' warning appears only where every jump of the leg is listed and known not to be scoopable.
- The next stop is the one raised row: larger name, its copy icon beside it, "N jumps away". Other stops: copy icon and Make next on hover. Skip and Back buttons go (voice keeps them).
- Inhabited systems get a second line: allegiance and government, security, population, controlling faction and its state.
- No "On your clipboard" line: the copy icon turns to a check; a failed copy is the red error.

## Report

The three children (astromech-irrv the title, astromech-woqv the route data, astromech-qwvs the timeline) were done, so this item was the check that the page as a whole matches the Design, plus one gap.

- The gap: `NeutronFile.svelte`, the neutron star file's status in the title's Worked out dropdown, is part of the page (Title > PlotSetup > NeutronFile) but was not on the page-sizing check, and set its own font size and a 24rem progress-bar cap. It now takes the Label role (keeping the dropdown's colour) and the bar fills the dropdown; it is enrolled for both parts in `app/eslint.config.js`. Every route file is now either checked (View, Title, Timeline, PlotSetup, ImportDialog, NeutronFile) or exempt (Strip, PlotBar). `Plot.svelte` is in neither list: it is only the header's plot panel inside PlotBar, so a frame part like PlotBar; the mission's final enrolment (astromech-gvy0) should add it to PAGE_SIZING_EXEMPT.
- Captures in the browser preview, height scale on (it is always on): `?core=ready&route=timeline`, `route=exact`, no route, and the Worked out dropdown open, each at 2560 x 1440 and 1100 x 700. Against the Design: the title reads Sol to Colonia with the destination editable, Plotter and Worked out dropdowns, one Replot from here, ? and ⋯; the timeline's nodes are the star-type Glyphs; the flown leg opens one row a jump with your row highlighted and pinned; the next stop is the raised row with its copy icon and "9 jumps away"; inhabited stops carry the second line; the exact route lists every star with fuel stops. Nothing clips at 1100 x 700 (the timeline scrolls under the status bar); at 2560 x 1440 the page stays sparse on Board. Make next shows on hover; copy icons show at rest, as astromech-qwvs shipped and its review approved.
- Checks: `pnpm lint`, `pnpm check` (0 errors, 0 warnings), and `vitest run src/modules/route src/lib/route.test.ts eslint-rules` (9 files, 106 tests) pass.
