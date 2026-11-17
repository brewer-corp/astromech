---
schema: atelier.item
schema_version: 3
id: astromech-z4vm
title: Header tabs run off a narrow window at large Text sizes, and the page scrolls sideways
status: building
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A1bf79357-3f85-4f54-bfda-685621caf9b4
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T12:43:21.120641266Z
trivial: null
caused_by: astromech-b2cz
created_at: 2026-11-17T12:43:21.061892208Z
---

## Goal

The header fits the window at every Text size. At 145 percent in a window 1,100 px wide, the ten top tabs run 292 px past the window's right edge on every page and the whole page scrolls sideways (astromech-fzhp's implementer, 19 Nov 2026; astromech-b2cz reported the same with eight tabs). A commander on a 1440p screen starts at 145 percent, so this is what they get when the window is not maximised. The header's second tab row moves the tabs under the wordmark but does not wrap them.

## Done when

- [ ] A failing browser test first: at each Text size step, at the narrowest window the app allows, the document does not scroll sideways and every top tab is reachable.
- [ ] When the tabs do not fit on their row they wrap onto further rows or are reached another way the Report justifies against DESIGN.md's Two Tab Rows Rule; no tab is hidden without a way to reach it, and none is clipped.
- [ ] DESIGN.md's Frame and height budget say what the header does at a narrow window at each Text size step, with the measured heights, and what that leaves the page at 1100 x 700 at 145 and at 180 percent.
- [ ] Captures at 1100 x 700 at 100, 145 and 180 percent. `pnpm check`, `pnpm lint`, `pnpm test` pass.

## Scope

The header at narrow widths. Each page's own narrow layout at large text is its own item's.
