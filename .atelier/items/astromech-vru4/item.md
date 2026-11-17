---
schema: atelier.item
schema_version: 3
id: astromech-vru4
title: 'Glance test: Station''s main value is outshouted by For sale here''s larger ranked profit'
status: approved
mission: astromech-xajc
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A4b5e4767-a3f7-4c82-90a8-fedc5b92af7d%3Adelegate-task%3A852bf5eb-9612-4735-9473-64e161e9aff7
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T15:10:56.301215159Z
trivial: null
caused_by: astromech-7vxi
created_at: 2026-11-17T15:07:41.712215369Z
---

## Goal

On Station, both glance-test readers (astromech-crxa, 17 Nov 2026) took For sale here's +52,506 cr/t on Tritium as the page's main value, not the hold's +3,921 cr/t on Gold. astromech-7vxi makes the first cargo line's gain the main value in Orange Lit and For sale here's profit the ranked value, also orange, both in Value L, so the larger ranked figure further down outshouts the main value.

## Done when

- [x] Decided by the lead of astromech-xajc on 19 Nov 2026: change the design. On Station the hold's gain per tonne on the first cargo line is the page's one main value, in Value XL, Orange Lit. The ranked values in For sale here and the lists below keep Orange as ranked values but at row size (the grouped row's value size), so no figure further down is as large as the main value. The Station guide page says what leads. The change is built so, with before-and-after captures at 2560 x 1440 and 1100 x 700, and `pnpm check`, `pnpm lint` and `pnpm test` pass.
- [x] The page is recaptured at 2560 x 1440 at 145 percent the way the glance test takes its captures, and a fresh agent given only that capture and the glance test's prompt answers 'main value' as the answer sheet's row for this page expects; the readers' answers and marks are recorded beside the captures in `docs/design/glance-vru4/README.md`. astromech-crxa copies them into `docs/design/final/glance-test.md` when it resumes.

## Scope

Station, docked, in `app/src/modules/station/`, and DESIGN.md if a rule changes. The other two answers on this page passed and stay as they are.

## Context

astromech-crxa's answer sheet, both readers' answers and the analysis are in `docs/design/final/glance-test.md` (row 4, and "What failed and why"). The capture: `docs/design/final/station-2560x1440.png`. This item blocks astromech-crxa: its criterion is met only when every page passes or Sumeet takes the page out of scope.

## Report

Both fresh glance readers now name the hold's +3,921 cr/t on Gold as Station's main value.
Before the change, both readers named For sale here's +52,506 cr/t.

The size change alone was not enough. One reader (aab26d8de91da8ed5) saw the gain at Value XL
in its old place, below the fact list. It split the main value between the title "Test Orbital"
and the gain. So the change also moves the Market up, as astromech-8jqg did on Now.

What changed:
- `app/src/modules/station/Market.svelte`: the hold's first gain is `type-value-xl main`
  (28 px, Orange Lit). The hold's other gains and For sale here's ranked profits stay Value L
  (18 px), the profits in Orange.
- `app/src/modules/station/View.svelte`: the Market section comes straight after the station's
  name. The fact list follows it under a new section heading, "The station". Outfitting and
  "Where did I see…" stay where they were.
- `DESIGN.md`: the Value XL row's use now covers a page's main value that sits in a list row.
- `docs/guide/now.md`: Market comes before The station, as on the page. It says the first
  cargo gain is the largest figure and why the profits are orange.
- Captures in `docs/design/glance-vru4/`: `before/` is main, `after/` is the change, each at
  2560 x 1440 (145 %) and 1100 x 700 (100 %), with the glance test's query and clock. The audit
  finds nothing clipped, overlapping, sideways, too small, low in contrast or far in any capture.
- `pnpm check`, `pnpm lint` and `pnpm test` (922 tests) pass. One earlier `pnpm test` run lost
  its browser connection in `now/View.browser.test.ts`, a file this change doesn't touch, while
  the machine's load average was about 11. It passed on the next run.

The readers' answers and marks are in `docs/design/glance-vru4/README.md`. Reader 1 failed
needs you: it named the market's 4-day age, where the sheet says none. Round 1's reader did the
same before this change. That question is outside this item's Done when, but astromech-crxa
needs all three answers to pass, so the lead should weigh it.

Following the lead's revised line 2, `docs/design/final/glance-test.md` is not touched here.
astromech-crxa copies the answers in when it resumes.
