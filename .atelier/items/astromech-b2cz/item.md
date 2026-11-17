---
schema: atelier.item
schema_version: 3
id: astromech-b2cz
title: Text size you choose replaces the window zoom
status: ready
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-n41x
implementers: []
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.201058589Z
---

## Goal

A bigger window shows more, and resizing never changes the size of the text. The window's zoom from its height (astromech-k6pl) goes; the commander's Text size sets one root size, as on any web page. Sumeet, 16 Nov 2026: the auto-zoom "feels very wrong".

## Done when

- [ ] `tokens.css` and the type role classes are in rem, and no component sets a px font size or spacing that should scale with text (borders and 1 px rules stay px). `html` font size comes from Text size alone; `scale.svelte.ts`'s window formula, `--win-h`, `--win-w` and the zoom corrections in `tip.ts`, Dialog, Menu, Picker and My controller's sheet are gone.
- [ ] Text size has six steps, 100, 115, 130, 145, 160 and 180 percent of a 16 px root, in Settings > General, live and persisted as today. 145 percent at 2560 x 1440 matches what the approved captures show (the old 1.44 zoom).
- [ ] First run only: with no saved choice, the step nearest the display's height divided by 1000 (the screen's, never the window's) is saved as the choice. After that only the commander changes it. A test covers a 1080, a 1440 and a 2160 display and that a resize changes nothing.
- [ ] The `@container win` rules are in rem and still switch where the layout needs them at 100 and 180 percent.
- [ ] Every page checked in the preview at 1100 x 700 and 1920 x 1080 at 100 percent, 2560 x 1440 at 145 percent and at 100 percent, and 3840 x 2160 at 180 percent: nothing clipped, no sideways scroll. Past its natural width a layout leaves the rest of the window empty; it does not stretch.
- [ ] Captures of the Sell finder at 2560 x 1440 at each of the six steps are in `docs/design/text-size/`, for Sumeet to see the choice.
- [ ] The user guide's Settings page says what Text size does. `pnpm check`, `pnpm lint`, `pnpm test` pass.

## Scope

Sizes and the setting. Colours are astromech-3kf5. The Windows check of the result is astromech-3340.

## Context

DESIGN.md revision 5 (astromech-n41x) states the rule. The zoom was a shortcut to leave px tokens alone; media queries, `vh` and tooltip placement all needed workarounds for it.
