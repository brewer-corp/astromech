---
schema: atelier.item
schema_version: 3
id: astromech-fom5
title: 'Finders open on their results: range and origin fold into the summary line'
status: approved
mission: astromech-xajc
labels:
- feature
- size-m
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A7c12354d-ad37-47bf-b04d-8e99a1778517
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

- [x] On every finder that has them, Range and From are inside the collapsed filter panel that CHANGE opens (`SearchFilters.svelte`), and the summary line states them as it does today. At rest the page is: the title with its main input, the summary line with CHANGE, the results.
- [x] The main input (the commodity, the module, the service) stays on the title row, and the figure it is measured against ("100 t of Painite aboard") stays beside it.
- [x] Sell opens with at least 4 whole result rows at 1100 x 700 at 100 percent and at least 10 at 2560 x 1440 at 145 percent, measured in the preview; DESIGN.md's rows-visible figures are updated with the measurement.
- [x] Changing Range or From behaves as the other filters do: edits search at once, and the panel closes only on DONE, Escape or CHANGE. A finder opened from a link with a range or origin in it shows that in the summary.
- [x] Browser tests on recorded markets cover opening the panel to change the origin and the summary naming a new range while the search runs. Before-and-after captures of Sell and Mining hotspots at both sizes. The Places guide page is updated.

## Scope

Where the range and origin controls live. The grouped row itself is done (astromech-3ca6).

## Context

`docs/design/grouped-rows/sell-1100-after.png` shows the problem; `docs/design/nav-spike/tone5-sell-1100.png` is the approved mockup, which drew no Range or From rows. DESIGN.md revision 5's Collapsed filters and Lookup result row sections.

## Report

Finders now open on their results. Sell anywhere opens with 5 whole rows at 1100 x 700 at
100 percent (3 before) and 11 at 2560 x 1440 at 145 percent (9 before). I measured both in the
preview (`?core=ready`, no route sidebar) with `scripts/ui`, counting rows whose bottom is above
the status bar. The recorded markets give Sell near only 8 rows, so the measurement uses Sell
anywhere, as astromech-3ca6's did. DESIGN.md's rows-visible figures now say 5 and 11.

**`pnpm test` has one failure, the same one main has:** `App.browser.test.ts` "docks from the
dock width". astromech-7obs owns it, and another thread is building it, so I left it alone
on this branch. For 7obs, from measuring it here:

- Sell is no longer the widest finder row beside a side panel. Its row needs 672 px. Mining
  hotspots needs 856, because its own group is text and it has a trip. Measured docked with a
  row selected, Mining hotspots needs 1,659 px at 100 percent and 1,655 at 180. Sell needs
  about 1,476. A dock width measured on Sell would stack Mining hotspots' panel on every window
  from about 1,476 to 1,659 px.
- The test's own arithmetic is off too. It assumes the folded page column grows by exactly the
  sidebar's width. The dock also has a 12 px seam, and `--side` (3vw) narrows with the window.
  Narrowing the folded window until its column matches the docked one fixes that.
- At 1,659 the cold `#/route` test needs a saved 145 percent instead of 115, since 1920 / 1.15
  = 1,670 would dock.
- The patch I wrote for this (DOCK 1659, the test on Mining hotspots) is not on this branch.

What changed:

- **Range and From are filters.** `SearchFilters` takes an `origin` snippet, shown after the
  finder's own filters (`extra`) and before the shared ones. CHANGE shows whenever there is one.
  `SearchKit` puts the origin box there and draws its query band only when a finder passes a
  `header` or `lead`. Market's Range choice moved from `lead` to `more`. The summary already
  named both ("Within 100 ly of Sol", "Best prices anywhere · distances from Sol").
- **Every SearchKit page changes, not only the Places finders.** Colonisation's where-to-buy,
  Powerplay's Where to work and Nearby also have the origin box behind CHANGE now. In the
  panel the box is 20rem, as wide as a finder's main input, and it shrinks in a narrow host.
  Before that fix, Powerplay's column at 1100 x 700 at 145 percent overflowed by 96 px. I
  checked Powerplay at 100 and 145 percent. I did not open Nearby in the Engineering side
  panel; it uses the same shrinking rule.
- **Edits behave like the other filters.** Range and origin edits search at once, and the panel
  stays open. In the full app a Range change rewrites the address, but Places keys a page on its
  finder id, so the finder isn't remounted and the panel stays open. I checked that with
  `scripts/ui`. `OriginInput`'s Escape now closes only its suggestions while they are open;
  before, it would also have closed the panel. The "Next stop: Deciat" label is bindable and
  held by `SearchKit`, so it survives the box closing with the panel.
- **What stays above the summary.** The main input and "100 t of Painite aboard" stay on the
  title row. These also stay in the query band, because they qualify what is searched for:
  Outfitting's class, rating and mount; Sell my cargo's hold; and the description lines on
  Raw sites and Faction states. Those pages are not literally "title, summary, results", and
  DESIGN.md's Collapsed filters says so.
- **Opened from a link.** No link carries an origin today, and I didn't add one. A range in the
  address (`?range=anywhere`) shows in the summary ("Cheapest anywhere"); the Buy test covers
  it. Colonisation's `start` (the site's system) shows in its summary; tested.
- **Tests on the recorded markets.** Market now covers opening the panel and picking Anywhere
  while Ardent's reply is held. During the search the summary names the new range and the
  panel stays open. It also covers changing the origin to Deciat inside the panel, where the
  summary says "Within 100 ly of Deciat" while the search runs, and Escape closing the panel
  from the box. Tests that clicked Range or the origin box now open CHANGE first.
- **Docs.** DESIGN.md has the Collapsed filters text and the rows-visible figures. The Places
  guide puts Range, From and Around under Change. Colonisation's hint, its guide page and
  Using the app say From is under Change. Captures are in `docs/design/finder-fold/`: Sell
  near, Sell anywhere and Mining hotspots at 1100 x 700 and 2560 x 1440 at 145 percent, before
  and after, plus Sell with CHANGE open.

Not done: the gap between the title row and the summary (about 57 px at 100 percent) is wider
than the approved mockup's (about 40). The row targets are met without changing it.

Checks: `pnpm lint`, `pnpm check`, `pnpm build` with `scripts/check-dist.py`,
`scripts/check-guide.py`. `pnpm test`: 728 of 729 pass; the one failure is the dock test above.
