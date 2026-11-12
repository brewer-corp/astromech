---
schema: atelier.item
schema_version: 3
id: astromech-irrv
title: 'Route page: the title is the plotter'
status: building
mission: astromech-xajc
labels:
- size-m
priority: high
blocked_by:
- astromech-k6pl
- astromech-mwh5
- astromech-woqv
- astromech-z5oh
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3Aaf4279c3-1204-452d-9106-7f433154a04e
- 01a1cf5f-c437-7b61-8611-c1fafc06dd46
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3Aaf4279c3-1204-452d-9106-7f433154a04e
ready:
  by: conversion
  at: 2026-11-07T03:00:11.210775156Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:11.210775156Z
---

## Goal

The Route page's title is where you plot: from, the destination editable in place, Plotter and Worked out dropdowns (the neutron star file's status, download and check inside), Replot from here, the range-changed line, and the ? and ⋯ menu. The plot side panel, the import box and the on-page help go.

## Done when

- [x] Plotting, replotting, changing the engine and downloading the neutron star file work from the title in the preview, with their busy and error states.
- [x] Import, Open on Spansh and Clear work from the ⋯ menu; import opens a dialog.
- [x] The header's plot popover and the page agree (one plot setup).
- [x] Route tests updated.

## Report

The Route page's title now plots: "Sol to [destination]", the destination edited in place with
Spansh suggestions, Plotter and Worked out dropdowns, one primary button, the range-changed line,
and ? and ⋯ at the end of the row. The side panel's import box and the on-page galaxy-map help
are gone. Commit f7050519.

- One plot setup: `app/src/lib/plot.svelte.ts` holds the plotter, engine, efficiency, refuel,
  the ship, the neutron star file's status and a shared busy flag. `PlotSetup.svelte` shows it
  as the two dropdowns, in the title and in the header's plot panel (`Plot.svelte` now uses it
  instead of its own Choice and fields). The page adopts what the route followed was plotted
  with (plotter, efficiency or refuel) once per route; it does not adopt the engine, because a
  route records `spansh` whenever the file was missing. A plot running in one place disables
  the other's button and says where it runs.
- Worked out: On this PC (disabled until the file is there) or By Spansh, with the file's
  status, Download and Check under the options, and Efficiency when Spansh will plot. Exact
  routes show "Worked out By Spansh" as a fact, not a cell. Refuel at every scoopable star sits
  under Plotter for Exact.
- The button is Replot from here when the box names the route's own destination, else Plot
  route. Replot calls `route_plot` with the current setup and the ship's range now, not
  `route_replot` (which keeps the stored range): with the range line saying the range changed,
  replotting at the old range would be surprising. Escape in the box puts the route's
  destination back.
- ⋯ menu: Import a route… (a modal dialog; a failure stays in it with the text), Open on
  Spansh, Clear route. Open on Spansh opens the destination's Spansh page (search by name):
  astromech keeps no Spansh job id, so it can't open the plotted route itself. Worth a call
  from Sumeet if he expected the route.
- New shared pieces: `lib/ui/Picker.svelte` (dropdown whose options carry a Label line and extra
  content below), `lib/ui/Menu.svelte`, `lib/ui/Dialog.svelte` (native modal `<dialog>` with
  `aria-modal`, so Ctrl+K stays out), and the `more` and `help` glyphs (DESIGN.md Icons updated).
- The preview's `route_plot` now honours `engine: "spansh"` and goes through `previewGate`, so
  `?slow=` and `?fail=route_plot` work.
- Left for astromech-qwvs: the next-stop block, Copy/Skip/Back and the stops list are unchanged
  apart from Clear route moving to ⋯; the page is not yet on a layout component. The title's
  own files (Title, PlotSetup, ImportDialog) are enrolled in both parts of the page-sizing check
  and pass. Expect a merge conflict in `View.svelte` with qwvs.

Checks: `pnpm lint`, `pnpm check`, and the full vitest run (619 tests, 83 files) pass;
`scripts/check-guide.py` passes. New tests in `Title.browser.test.ts` cover plotting, replot,
busy, error and Try again, the file download and engine change, the range line, the shared setup
with the header panel, and the menu; View, PlotBar, NeutronFile and staleMocked tests updated.
Looked at in the preview at 2560 x 1440 and 1100 x 700 (Worked out open, import dialog); nothing
clipped. Note for others on this machine: parallel worktrees collide on vitest's browser port
63315; I ran tests with a scratch config setting `browser.api.port`.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-ok29 (Route page redesign: the title is the plotter, the route a timeline)
