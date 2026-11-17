---
schema: atelier.item
schema_version: 3
id: astromech-7obs
title: 'The Sell dock-width test fails on main: the sidebar''s dock width was measured before the grouped rows'
status: building
mission: astromech-xajc
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A92a208ce-df1e-4251-9d03-3358e03af3cf
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:13:15.774735374Z
trivial: null
caused_by: astromech-3ca6
created_at: 2026-11-17T09:13:15.683726201Z
---

## Goal

`pnpm test` passes on main, and the route sidebar docks at the width the pages need today. astromech-uc7i measured the dock width (1,756 px at 100 percent Text size) against the one-line Sell row and left a browser test in `App.browser.test.ts` that fails when the row's width changes. astromech-3ca6 then rebuilt the row in groups, and the test has failed on main since 435705e (seen by astromech-uzg6's implementer and reviewer, 736 of 737 passing).

## Done when

- [x] The dock width is measured again against the widest finder row with a result selected (Mining hotspots today), at every Text size step, and set to the narrowest width at which the rows stay clear of the detail panel beside the sidebar. The test passes because the number is right, not because the test was loosened.
- [x] The Report gives the new width and what it means for the windows that matter: a maximised 2560 x 1440 window at 145 percent (about 1,765 px of layout) docked by about 10 px before; say what the margin is now, and whether a 1920 x 1080 window at 100 and at 115 percent docks.
- [x] If the grouped row lets the sidebar dock on narrower windows, it does. DESIGN.md's Route sidebar section has the measured width.
- [x] `pnpm test`, `pnpm lint` and `pnpm check` pass.

## Scope

The dock width and its test. The finder header above the rows is astromech-fom5, which does not change a row's width.

## Report

**The sidebar now docks from 1,659 px at 100 percent Text size, down from 1,756.** This number comes from Mining hotspots, which is now the widest finder row with a result selected. Sell's grouped row would allow 1,475, but at that width Mining hotspots' panel would go below its rows on windows from 1,475 to 1,659 px. As the lead asked, the dock width and the test follow the widest finder, and the first Done when line now says so.

Measured docked, with a fresh render for each probe and a binary search on the window, as the narrowest window where the panel sits beside full rows:

| Finder | 100 % | 145 % | 180 % (px at 100) |
|---|---|---|---|
| Mining hotspots | 1,659 | 1,656.6 | 1,655.0 |
| Raw sites | 1,539 | 1,536.6 | 1,535.0 |
| Outfitting, Shipyard | 1,491 | 1,488.3 | 1,486.7 |
| Sell, Buy, Sell cargo, Faction states | 1,475 | 1,472.4 | 1,470.6 |
| Material traders, Services | 1,355 | 1,352.4 | 1,350.6 |

Mining hotspots at every step: 1,659 (100), 1,658.3 (115), 1,656.9 (130), 1,656.6 (145), 1,655.6 (160), 1,655.0 (180). Below each number the panel goes under the rows. In every case the row stays full and is never compacted. Before the grouped rows (at `435705e7^1`), Sell, Buy and Sell cargo were within about 25 px of the widest, so Sell was a fair reference then.

What this means for the windows that matter:

- **2560 x 1440 at 145 percent** (1,765.5 px of layout) docks with about 106 px to spare, up from about 9.
- **1920 x 1080 at 100 percent** docks with 261 px to spare.
- **1920 x 1080 at 115 percent** (1,669.6 px) now docks, with about 10 px to spare. It did not dock before.
- 1920 at 130 percent (1,477 px) does not dock.

**Changes:**

- `DOCK` is now 1659 in `app/src/lib/ui/layout/layout.ts`, and DESIGN.md's Route sidebar section has the measured widths.
- The dock test in `App.browser.test.ts` is now two tests:
  - The first checks all ten Places finders at the dock width, at every Text size step: each is docked, with its panel beside full rows. A finder that grows wider than Mining hotspots fails it, which covers astromech-srab's type-icon slot.
  - The second checks that the dock width isn't loose. At each step, Mining hotspots' panel goes below its rows once the page column is the docked one less 6 px at 100 percent. It narrows the folded window until the column matches, because the page gutters (3vw) shrink with the window.
  - With `DOCK` at 1655 both tests fail, at 1668 the second fails, and at 1659 both pass.
- The cold `#/route` test now saves 130 percent instead of 115, because 1920 px at 115 percent docks now.

**Checks:** `pnpm test` passes (89 files, 752 tests), and `pnpm lint` and `pnpm check` are clean.

**Not fixed here, needs filing:** a compact row can get stuck at 160 percent. With the sidebar folded and Sell's panel beside its rows, a page column of 1,823 px left the content track a fraction of a pixel short of the row's 672 px. The rows went compact, the track shrank to the compact rows' 487 px, and they stayed compact after widening to 2,130 px. The cause is that SearchKit measures `.out`, which in `Split`'s side layout is a max-content track that depends on whether the rows are compact. It doesn't touch the dock width: at the dock width the 160 percent column is 2,655 − 512 px wide, far from the edge. `atelier new` refuses on an item branch, so this went to the lead to file.

