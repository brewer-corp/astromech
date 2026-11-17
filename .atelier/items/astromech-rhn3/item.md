---
schema: atelier.item
schema_version: 3
id: astromech-rhn3
title: Finder rows that go compact stay compact after the window widens
status: changes_requested
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by:
- astromech-srab
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A17040ddb-fde9-4b03-9dbd-722b1d3ea977
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:35:15.494625804Z
trivial: null
caused_by: astromech-3ca6
created_at: 2026-11-17T09:35:15.407940249Z
---

## Goal

A finder's rows return to their two-line form when there is room again. Found by astromech-7obs's implementer on 18 Nov 2026: at 160 percent Text size with the sidebar folded, Sell's rows switch to their one-line compact form at a page column of 1,823 px and stay compact after the window is widened to 2,130 px. `SearchKit.svelte` measures `.out`, whose width in `Split`'s side layout shrinks to fit the rows, so once the rows are compact the measured width never grows back.

## Done when

- [x] A failing browser test first: the rows go compact when the page column narrows, and return to two lines when it widens again, at 160 percent with the sidebar folded.
- [x] The fix is at the cause: SearchKit measures the space its rows are given, not an element that shrinks to fit them.
- [x] The dock-width test still passes.

## Scope

The shared result row's compact switch.

## Report

**A finder's rows now go back to two lines when the window widens.** At 160 percent with the sidebar folded, Sell's rows go compact in a 1,100 px window and return to full rows at 2,130 px. Before the fix they stayed compact.

**The cause was in two places, not one.** SearchKit measured `.out`, which sits in `.kit { width: fit-content }`. That shrinks to fit the rows everywhere, not only in `Split`'s `minmax(0, max-content)` track. So a kit with no side panel (Powerplay's Where to work, Nearby, Colonies, colonisation's Where to buy) stuck the same way once its column had been narrow.

**The fix:** SearchKit compares the full row's width (`fits`) against the width it is given, never its own width.

- In a ListDetail, `Split` reports the content's room through a new `onroom` callback, and `ListDetail` puts it in the detail context as `detail.room`. The room is Split's width, less the gutter and panel track when the panel is beside it. That formula is `besidePanel()` in `layout.ts`, which `stacks()` now uses too.
- Since `detail.min` is `fits`, side and compact now come from one measurement, so a row beside its panel is never compact. This also removes the fractional-pixel trigger astromech-7obs found at a 1,823 px column.
- Elsewhere the kit observes its parent element's content box (`contentRect.width`). The four parents named above are block or grid items that fill their column.
- DESIGN.md's Lookup result row says the room is the width the list is given, and that a row beside a side panel is never compact.

**Tests:**

- `App.browser.test.ts`: "goes back to full rows when the window widens after they went compact, at 160 percent with the sidebar folded". It failed before the fix with `expected true to be false` on the "full rows again at 2,130 px" line.
- `SearchKit.browser.test.ts`: "goes back to the groups when its column widens after the rows went compact" (600 px then 800 px, the Sell fixture). It failed before the fix, and the Trip head never came back.
- Both tests were committed red in 86454016, before the fix in 59bcc844.
- The narrow-column test no longer mocks `clientWidth` and `ResizeObserver`. It renders into a real 297 px element.

**Checks:** `pnpm test` passes (90 files, 755 tests), including both dock-width tests. `pnpm lint` and `pnpm check` are clean.
