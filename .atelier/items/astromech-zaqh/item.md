---
schema: atelier.item
schema_version: 3
id: astromech-zaqh
title: 'Sell finder: the side-panel scrolling test runs with the real design sizes'
status: ready
mission: astromech-xajc
labels:
- size-s
- bug
priority: medium
blocked_by: []
implementers: []
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

- [ ] The case runs with `tokens.css` loaded, as `App.browser.test.ts`, `MarketRows.browser.test.ts` and `colonisation/View.browser.test.ts` already do.
- [ ] It asserts what the commander needs from a panel taller than the window: after selecting a result, the panel's two actions (Set destination and Add stop) can be scrolled into view inside the page column and are not covered by the status bar or the route strip; it does not ask the whole panel to fit.
- [ ] If the page cannot reach the actions with the real sizes, that is the bug: it is fixed in the layout (`app/src/lib/ui/layout/`) or the finder, with this case failing first.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass. One full test run at a time: each agent here has 4 GB.

## Scope

This one test and anything it proves wrong. No visual change is intended.

## Context

The finding came from astromech-srab's review round 1. astromech-gvy0 enrols the Places finder files in the font-size part of the page-sizing check, so it waits on this.
