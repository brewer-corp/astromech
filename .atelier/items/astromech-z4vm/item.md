---
schema: atelier.item
schema_version: 3
id: astromech-z4vm
title: Header tabs run off a narrow window at large Text sizes, and the page scrolls sideways
status: approved
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

- [x] A failing browser test first: at each Text size step, at the narrowest window the app allows, the document does not scroll sideways and every top tab is reachable.
- [x] When the tabs do not fit on their row they wrap onto further rows or are reached another way the Report justifies against DESIGN.md's Two Tab Rows Rule; no tab is hidden without a way to reach it, and none is clipped.
- [x] DESIGN.md's Frame and height budget say what the header does at a narrow window at each Text size step, with the measured heights, and what that leaves the page at 1100 x 700 at 145 and at 180 percent.
- [x] Captures at 1100 x 700 at 100, 145 and 180 percent. `pnpm check`, `pnpm lint`, `pnpm test` pass.

## Scope

The header at narrow widths. Each page's own narrow layout at large text is its own item's.

## Report

**At 145 percent in a 1100 px window, every top tab now shows and the page no longer scrolls sideways.** When the stacked tab row doesn't fit, it wraps onto further 44 px lines. This costs page height. At 1100 x 700 the header is 220 px at 145 percent, where the old fixed 6.75rem header was 157 px, and 273 px at 180 percent. That leaves the page 318 px at 145 percent and 226 px at 180, or 382 and 305 on a page with no page tabs such as Places. The lead should weigh those numbers if a menu looks better.

**Wrapping against The Two Tab Rows Rule.** The rule counts levels: top tabs, then one row of page tabs. A wrapped line of top tabs is the same level, styled the same way, and holds the same ten areas. So this is still two tab rows. A More menu would keep the header at 64 + 44 px, but it would hide some areas behind a second click, and the first Done when line asks that every tab stay reachable. DESIGN.md now says this in the rule, in Tabs ("Maximum two levels") and in the Don'ts.

**Changes, in three commits:**

- `238a7f69`: the failing test, in `app/src/App.browser.test.ts` under "header in a narrow window". At every Text size step it checks the Materials page in an 800 x 500 window (the minimum `app/src-tauri/src/host.rs` allows), in a 1100 x 700 window, and at 800 x 500 with Finish setup showing. At each step it checks four things: the document is no wider than the window; every top tab is drawn inside the header and the window; no tab's label is clipped; and the wordmark, Find or plot and Help stay on one row (not checked with Finish setup). Before the fix all three cases failed: `100 %: the document scrolls sideways: expected 959 to be less than or equal to 800`, and in the 1100 px window `115 %: ... expected 1116 to be less than or equal to 1100`.
- `fa5645d3`: the fix, in `app/src/App.svelte`'s styles.
  - Stacked, the header's height is `auto`, and the tab `nav` wraps onto 2.75rem lines. Each line keeps the tab's 2.125rem box centred.
  - Stacked, Find or plot's flex basis is its 11.25rem minimum, and it grows back to 20rem. Flex line-breaking uses the basis, so at 800 px and 160 percent the 20rem basis pushed Find or plot and Help onto lines of their own. The extra name-row check above caught this.
  - The width at which the header stacks is unchanged. With and without the fix it stacks below 1593, 1840, 2065, 2316, 2549 and 2878 px at 100 to 180 percent in the test Chromium, the same whichever way the window is resized. That is within 9 px of DESIGN.md's figures, which were measured elsewhere, so I left those as they were.
- `3c74daea`: DESIGN.md's Frame row and a new Narrow window paragraph under the Height budget, with the measured header for each step at 1100 px and at 800 px, and what that leaves the page. Round 1 corrected the 800 px figures at 130 percent: my first loop resized one window through the steps and read 130 percent before it settled, so each step is now measured in a fresh window once its fonts have loaded. Also the rule rewording above, and one sentence in `docs/guide/using-the-app.md`.

| Text size | 1100 px: tab lines, header | 800 px: tab lines, header |
|---|---|---|
| 100 | 1, 108 px | 2, 152 px |
| 115 | 2, 175 px | 2, 175 px |
| 130 | 2, 197 px | 2, 197 px |
| 145 | 2, 220 px | 3, 284 px |
| 160 | 2, 243 px | 3, 313 px |
| 180 | 2, 273 px | 3, 352 px |

**Captures**, in `docs/design/`: `header-wrap-1100x700-{100,145,180}-before.png` and `-after.png`, and `header-wrap-800x500-180-after.png`. The 145 percent before capture shows the tabs running to 1,392 px in a 1,100 px window.

**Checks:** `pnpm check` and `pnpm lint` are clean, and `pnpm test` passes (97 files, 900 tests). `scripts/check-guide.py` passes.

**Not fixed here, for the lead to file** (`atelier new` refuses on an item branch):

1. The window allows 800 x 500 (`host.rs` `min_inner_size`), but DESIGN.md's minimum is 1100 x 700. At 800 x 500 the Materials page keeps 157 px at 130 percent, 54 at 145 and 8 at 160. At 180 percent the frame's fixed parts are taller than the window: header 352, route strip 65, page tabs 79 and status bar 58 add up to 554 px. The status bar then covers the page tabs (`header-wrap-800x500-180-after.png`). Either raise `min_inner_size` to match the design, or design a frame for that size.
2. At 800 x 500 and 180 percent with Finish setup showing, the name row takes a second line, because the wordmark, Find or plot at its minimum, Finish setup and Help need 896 px. Nothing is clipped and nothing scrolls sideways, but the header grows by one more line. Item 1 decides whether this matters.
3. In the same 800 px capture, the status bar cuts off its cargo figure ("Cargo 120 | 256" runs off the edge). That is the status bar's narrow layout, not the header's.

### Responses

1-8w81.1: fixed in 6903db57 (800 px at 130 percent is 2 lines and 197 px; the status bar covers the page tabs only at 180 percent; DESIGN.md and the Report now give the page height at 130, 145 and 160)
