---
schema: atelier.item
schema_version: 3
id: astromech-zaqh
title: 'Sell finder: the side-panel scrolling test runs with the real design sizes'
status: in_review
mission: astromech-xajc
labels:
- size-s
- bug
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A5ab04e5a-44e4-401e-8561-ac746ee674c4
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-17T10:04:13.661674985Z
---

## Goal

The Sell finder's side-panel test checks what the commander sees. `app/src/modules/places/Market.browser.test.ts`'s case "fills the side panel from the selected result: every fact under its head, and its two actions" (line 55, at 1100 x 500 from line 58) runs without `tokens.css`, so its sizes are the browser's defaults, not the design's. With the real sizes the panel runs past the window's foot (574 px in a 500 px window at 145 percent), so the passing test proves nothing about scrolling to the panel's actions.

Taken from deferred finding astromech-srab/1-s950.2, in .atelier/items/astromech-srab/reviews/1-s950.md:

> Finding 2: Give the Market side-panel scrolling test the real design tokens
> Recommendation: defer
>
> `app/src/modules/places/Market.browser.test.ts:55-70` asserts panel scrolling without loading tokens.css. Its geometry therefore does not represent the app: adding only that import in a Vite transform makes the existing 1100 x 500 / 145 percent case fail with the panel bottom at 574 rather than within 500. This confirms the Report's reason for isolating the new row test. The defect predates this change and does not block the folded-row fix, but the existing passing case cannot establish the scrolling behavior it claims. Load the actual tokens and check the intended behavior for a panel taller than the viewport, including reaching its actions, rather than relying on the unstyled dimensions or requiring an arbitrarily tall panel to fit wholly on screen.

## Done when

- [x] The case runs with `tokens.css` loaded, as `App.browser.test.ts`, `MarketRows.browser.test.ts` and `colonisation/View.browser.test.ts` already do.
- [x] It asserts what the commander needs from a panel taller than the window: after selecting a result, the panel's two actions (Set destination and Add stop) can be scrolled into view inside the page column and are not covered by the status bar or the route strip; it does not ask the whole panel to fit.
- [x] If the page cannot reach the actions with the real sizes, that is the bug: it is fixed in the layout (`app/src/lib/ui/layout/`) or the finder, with this case failing first.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass. One full test run at a time: each agent here has 4 GB.

## Scope

This one test and anything it proves wrong. No visual change is intended.

## Context

The finding came from astromech-srab's review round 1. astromech-gvy0 enrols the Places finder files in the font-size part of the page-sizing check, so it waits on this.

## Report

With the real sizes the commander can reach both actions, so there was no layout bug to fix; only the test was wrong. Commit 063463f1.

- The case moved from `Market.browser.test.ts` to a new file, `app/src/modules/places/MarketPanel.browser.test.ts`, which imports `tokens.css`. It renders `App`, not `Market` alone, because the route strip, the page column (`main`) and the status bar only exist in the frame. It runs at 1100 x 500 and 145 percent with `?route=exact`, so the strip is shown.
- It keeps the old case's content checks (the heading, demand, the four terms, the Following message) and replaces the "panel bottom within the window" check. It now asserts that the panel is taller than the page column (it is 1741 px against a column of about 260 px). Then, for Set destination and Add stop, it scrolls the button into view and asserts that the button lies inside `main`, below the strip and above the status bar, and that `elementFromPoint` at its middle is the button itself.
- The edges are compared with less than 1 px of slack. On the first run the button's bottom was 0.09 px past the status bar's top (453.70 against 453.61): the rem sizes at 145 percent fall on fractions of a pixel. Nothing is covered at that size.
- I checked that the case can fail. With `.col > main { flex: none }` the page column stops scrolling, and the height check fails (574 against 1741). With `margin-bottom: -3rem` on `main`, the status bar covers the column's foot, and the bounds check fails (499.7 against 431.6). Both changes were reverted.
- `Market.browser.test.ts` lost its now-unused `page`, `setTextSize` and `startPreview` imports. Its `afterEach` no longer resets the viewport or the scale, because no case in the file changes them and `test/transport.ts` resets the scale before each test.
- `pnpm check` and `pnpm lint` are clean. `pnpm test` passed 99 files and 906 tests, run once.

