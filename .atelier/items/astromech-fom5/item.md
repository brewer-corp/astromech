---
schema: atelier.item
schema_version: 3
id: astromech-fom5
title: 'Finders open on their results: range and origin fold into the summary line'
status: ready
mission: astromech-xajc
labels:
- feature
- size-m
priority: high
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:00:09.190860084Z
trivial: null
caused_by: null
created_at: 2026-11-17T08:59:59.810592407Z
---

## Goal

A finder opens on its results. After astromech-3ca6 the Sell finder at 1100 x 700 opens with 2 whole result rows (3 before the two-line rows, and 5 in the mockup Sumeet approved), because the Range choice and the From box sit open above the list and take about 170 px, while the summary line under them already says the same thing ("Within 100 ly of Sol"). astromech-drnv collapsed the other filters; these two were left out.

## Done when

- [ ] On every finder that has them, Range and From are inside the collapsed filter panel that CHANGE opens (`SearchFilters.svelte`), and the summary line states them as it does today. At rest the page is: the title with its main input, the summary line with CHANGE, the results.
- [ ] The main input (the commodity, the module, the service) stays on the title row, and the figure it is measured against ("100 t of Painite aboard") stays beside it.
- [ ] Sell opens with at least 4 whole result rows at 1100 x 700 at 100 percent and at least 10 at 2560 x 1440 at 145 percent, measured in the preview; DESIGN.md's rows-visible figures are updated with the measurement.
- [ ] Changing Range or From behaves as the other filters do: edits search at once, and the panel closes only on DONE, Escape or CHANGE. A finder opened from a link with a range or origin in it shows that in the summary.
- [ ] Browser tests on recorded markets cover opening the panel to change the origin and the summary naming a new range while the search runs. Before-and-after captures of Sell and Mining hotspots at both sizes. The Places guide page is updated.

## Scope

Where the range and origin controls live. The grouped row itself is done (astromech-3ca6).

## Context

`docs/design/grouped-rows/sell-1100-after.png` shows the problem; `docs/design/nav-spike/tone5-sell-1100.png` is the approved mockup, which drew no Range or From rows. DESIGN.md revision 5's Collapsed filters and Lookup result row sections.
