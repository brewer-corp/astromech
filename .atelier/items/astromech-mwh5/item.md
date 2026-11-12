---
schema: atelier.item
schema_version: 3
id: astromech-mwh5
title: 'Page layouts: Board, List with detail, Workbench and Form replace the width caps'
status: approved
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-b2k3
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A5aac3060-a7b6-4d8e-b38f-b0095d2b7464
ready:
  by: conversion
  at: 2026-11-10T06:04:03.897072879Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:54:57.572421657Z
---

## Goal

A page uses the window at any size without stretching rows. Today eight pages cap themselves at max-width 60rem and sit in the top-left of a big window, and Stored modules and On foot spread three columns across 1,800 px. Four shared layouts own widths, gutters and the detail panel, and a page picks one. A sparse page stays sparse: a layout places content well, it does not add any.

## Done when

- [x] app/src/lib/ui/layout/ has Board (tile grid), ListDetail (a list and a 320 px detail panel that fills from the selected row), Workbench (inputs left, result right) and Form, built to DESIGN.md revision 4.
- [x] One page per layout is converted and sets no width of its own: Materials (Board with a detail panel; its tiles are astromech-z5oh's), the Places finders through their shared FinderPage and SearchKit (ListDetail, the selected row's detail in the panel), Trade > Commodities (Workbench), Settings > General (Form).
- [x] At 1100 x 700 each converted page shows everything with nothing clipped (the detail panel moves below the content when there is no room); at 2560 x 1440 and 3840 x 2160 no action is further than one column from what it acts on. Before-and-after captures at 1100 x 700, 1920 x 1080 and 2560 x 1440 are in docs/design/.
- [x] One check, page-sizing, runs in pnpm lint and fails on a page module that sizes itself. It has two parts, each switched on per file from one list of files and globs in app/eslint.config.js, so a module can be enrolled a page at a time: width (a max-width or fixed width declaration in a file under app/src/modules/; a condition in a media query and max-width: var(--measure) on prose are allowed) and font size (any font-size declaration; added by astromech-z5oh). This item builds the check with both parts and its tests, and turns the width part on for the files it converts (for Places that is FinderPage and the finder pages, not Home.svelte, which astromech-gvy0 converts). Exempt from both parts for good: the Controls modules (controller, keybinds, presets, keytester), and the frame's parts that live in a page module (route/Strip.svelte and route/PlotBar.svelte), which are not pages.
- [x] pnpm check, pnpm lint and pnpm test pass.

## Scope

The layout components, the four conversions and the page-sizing check. The other pages convert in their own items (the coverage table in the strategy says which).

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. The layouts are visible in glow-up-materials.png (Board with detail) and glow-up-sell.png (ListDetail). UI rules: CONTRIBUTING.md (UI check; a visual change gets captures, not a test).

## Unknowns

Whether page-sizing is an ESLint rule over the Svelte style block or a small script that pnpm lint runs; either, as long as it fails the lint and has tests.

## Report

Four pages now use the window through a layout instead of a width cap, and a lint check stops a converted page from sizing itself again.

**Layouts** (`app/src/lib/ui/layout/`, to DESIGN.md revision 4):

- `Board` holds content plus an optional side panel. Its tile grid is `Tiles`: 200–240 px columns with 8 px gaps, a fixed `cols` count with an optional row-head column, or as many columns as fit.
- `ListDetail` holds a list and the side panel. A list inside it fills the panel through context (`listDetail.svelte.ts`), as SearchKit does, or the page passes the panel itself.
- `Workbench` puts the inputs in a 400 px column on the left and the result on the right, up to 1000 px.
- `Form` is one column of `--form-w` (900 px) and sets `--form-label` (200 px) for the rows.
- `Split` is shared by Board and ListDetail. The panel (320 px after a 1 px orange line) follows the content and the spare width goes after it. When the content can't keep its `min` width beside the panel, the panel moves below the content. A new selection then scrolls it into view. This is measured with `bind:clientWidth` because `@container` conditions can't take a variable.

**Pages converted:**

- Materials is on Board with the material (or the kind) in the panel. The tiles themselves are left to astromech-z5oh.
- The Places finders are on ListDetail through FinderPage and SearchKit. A result row can now be selected by clicking the row, or by clicking or pressing Enter on its name, which is a button. The selected row turns solid orange. The panel shows the name in white, where it is, the ranked value (white, `--fs-xxl`), every fact under its head (including those a compact row drops), and SET DESTINATION / ADD STOP. With nothing selected there is no panel.
- SearchKit outside a ListDetail (Nearby, Colonies, Powerplay targets, Colonisation) has no selection, as before.
- SearchKit rows are now as wide as their columns (place column 240–400 px, text columns up to 240 px), not stretched or capped at `--list-w`.
- Commodities is on Workbench: the lead links and the inputs in one column, then Find route, with the route on the right.
- Settings > General is on Form. The calibration dialog moved to `lib/ui/CalibrationDialog.svelte`, because its 600 px width is a component's, not the page's.

**page-sizing check:**

- The rule is `app/eslint-rules/pageSizing.js`, with two rules: `page-sizing/width` and `page-sizing/font-size`.
- Both are switched on per file from one list, `PAGE_SIZING` in `app/eslint.config.js`. Exempt for good, in `PAGE_SIZING_EXEMPT`: the Controls modules, route/Strip.svelte and route/PlotBar.svelte.
- The width part flags `width`, `max-width`, `inline-size` and `max-inline-size` that fix a length or a variable. It checks style blocks, `style:` directives and static `style` attributes.
- These pass the width part: query conditions, `max-width: var(--measure)`, `100%` and `none`, and widths that are shares or content sizes.
- The font-size part flags `font-size` and the `font` shorthand.
- Tests are in `app/eslint-rules/pageSizing.test.js`: RuleTester cases for both parts, plus config tests showing the check is on for enrolled files and off for unenrolled and exempt ones.
- Width is on for: materials/View, places/FinderPage, the nine finder pages, trade/View and trade/Route, and settings/View. Places Home.svelte is left to astromech-gvy0. Font-size is on for no file yet; astromech-z5oh turns it on.

**Captures:** `docs/design/layouts-{materials,sell,commodities,settings}-{1100,1920,2560}-{before,after}.png`, from the browser preview. They were retaken after merging main at 2afdcb11, which brought the root zoom, Tip, collapsed filters and the status bar: the "before" set is main at 2afdcb11, the "after" set is this branch.

- At 1100 x 700 no page scrolls sideways: `main` scrollWidth equals clientWidth on all four pages, Manufactured and Encoded, Outfitting and Services. The Materials and Sell panels sit below the content.
- At 2560 and 3840 the panel sits right after the grid or list, every row ends with its actions, and Find route ends the inputs.

**Calls I made inside Authority** (worth a look at review):

- The finder's 320 px main input width moved to a design-system rule in tokens.css (`.finder-main`), so FinderPage sets none.
- Tiles may narrow to 160 px in a stacked board at the minimum window. Otherwise a 5-grade grid runs off a 1100 px window. They are 200–240 px whenever the panel sits beside them.
- The panel follows the content rather than sitting at the window's right edge as in glow-up-materials.png. At 3840 x 2160 the edge position put it about 2,400 px from the row it describes.
- The panel's key value is white, per DESIGN.md Side panel; the mockup drew it Orange Lit.

**Left as it was:** settings/Callouts.svelte and ProspectorThresholds.svelte are on `--form-label` but not enrolled. Their 4.5rem number inputs would fail the width part; astromech-gvy0 enrols what is left. The commodities result keeps its own row layout, which astromech-el3i replaces.

**Round 1 fixes:** a panel action's note and TRY AGAIN now show under the panel's buttons. A hovered or focused selected row keeps On Orange icons. The panel stacks once the content would drop below its minimum beside the panel's full 345 px track.

**Tests:** a new browser test in `places/Market.browser.test.ts` selects a result and checks the panel's facts and Set destination. The guide's Places page (`docs/guide/places.md`, Results) now says how to select a result. `pnpm check` (0 errors, 0 warnings), `pnpm lint` and `pnpm test` (83 files, 617 tests after merging main) pass. After the merge, a result's name button takes Tip (`aria-label` plus `tip`) in place of `title`, as main's Tip test requires.


### Responses

- 1-3fjf.1: fixed in a9508970. A panel action's note, failure and TRY AGAIN now show under the panel's buttons; places/Market.browser.test.ts finds the note inside the panel, and failed before the fix.
- 1-3fjf.2: fixed in a9508970. A hovered selected row's icons are On Orange (rgb(12, 13, 15) in Chromium), and the row's icon buttons focus with an On Orange outline inset 2 px.
- 1-3fjf.3: fixed in a9508970. stacks() uses the panel's full track, PANEL_TRACK = 320 + 24 + 1; the new lib/ui/layout/layout.test.ts failed before the fix. On Sell at 1410 x 900 the panel stacks and the list keeps all nine columns.
- 2-ryk4.4: fixed in c223ace2. Split now waits on the panel element and scrolls once per selection; the Sell finder test selects a result in a 1100 x 500 window and checks the panel is on screen (failed before the fix). In the app at 1100 x 700 the first and second selections both bring the panel into view.
