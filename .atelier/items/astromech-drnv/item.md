---
schema: atelier.item
schema_version: 3
id: astromech-drnv
title: Finder filters collapse to their summary line
status: in_review
mission: astromech-xajc
labels:
- size-s
- feature
priority: medium
blocked_by:
- astromech-b2k3
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A92dbf88b-8c8d-4672-8898-5edce30d46ab
ready:
  by: conversion
  at: 2026-11-10T06:04:05.615896996Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:01.251561555Z
---

## Goal

A finder shows results first. Today every Places finder, Colonisation and Powerplay's 'Where to work' put up to five rows of filter controls above the first result. The one-line summary that already sits above the results ('Within 100 ly of Sol · medium or large pad · reports up to 30 days old') becomes the filters' resting state.

## Done when

- [x] At rest a finder shows the summary line with a CHANGE action and no filter controls. The thing being searched for (commodity, module, service) stays an always-visible input.
- [x] CHANGE opens the controls in place. Edits apply live as they do today (SearchKit searches again when a saved filter changes) and the panel stays open across them, so two filters can be changed without reopening it. It closes on DONE, on Escape, or on CHANGE again, and on nothing else: a search starting, finishing or failing never opens or closes it. This holds for every consumer.
- [x] SearchFilters.svelte owns the open or closed state and renders the summary from a summary prop its parent passes (SearchKit builds it from the answer, as it does now). A filter that differs from its default is always named in the summary. A failed search or a failed save leaves the panel as it was, with the error where it shows today.
- [x] Both kinds of consumer work: the ones that search on every change (SearchKit's finders) and the one with an explicit button (app/src/modules/trade/Route.svelte uses SearchFilters directly with FIND ROUTE), which may be wired at page level. There an edit changes the inputs FIND ROUTE will use and the summary line at once; FIND ROUTE works whether the panel is open or closed and leaves it as it is.
- [x] A browser test covers: open, change two filters, results update, DONE; a failing search with the panel open; and the explicit-search consumer.
- [x] Before-and-after captures of Places > Sell at 1920 x 1080 are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Mockup: docs/design/glow-up-sell.png. DESIGN.md's Lookup result row already specifies the summary line.

## Report

Finders now open on their results: the filters rest as the summary line with CHANGE.

- `app/src/lib/ui/SearchFilters.svelte` owns `open` and renders the `summary` prop with CHANGE
  (`aria-expanded`, accessible name "Change filters"). CHANGE toggles the controls in place
  below the line; DONE, CHANGE again, or Escape from CHANGE or any control closes them, and
  DONE and Escape return focus to CHANGE. Nothing else touches `open`, so searches and
  failures can't open or close it. A lookup with no filters shows the line without CHANGE.
- `SearchKit.svelte` moves SearchFilters from the query band to where the summary sat, above
  the column heads; the commodity, module or service and the From box stay in the query band.
  The summary is now built from the current filters, so it follows each edit at once, and it
  stays while a search runs and after one fails (it names the typed origin, else the last
  answer's system). The radius reached, hidden count and source notes still come from the
  answer. The filter-save error sits right under the line; the search error where it was.
- `Route.svelte` passes a filters-only line from the new `filterParts` in `search.ts`
  ("Medium or large pad · reports up to 30 days old"). An edit updates the filters Find route
  uses and the line at once; Find route leaves the panel open or closed.
- Summary gap fixed: a pad set to S over a bigger ship's was never named. It now reads "any
  pad" (unit test in `search.test.ts`).
- `Button.svelte` gained a bindable `ref` for the focus return.
- Behaviour change worth a look: the summary no longer disappears while a search runs; the
  "Asking…" line shows under it. The SearchKit test that asserted the old behaviour now asserts
  the new one.

Tests: SearchFilters (rest, open, DONE, Escape, CHANGE again); SearchKit "keeps the filters
open while two change and the results follow each, until DONE" and "leaves the filters open
when the search they started fails, and over TRY AGAIN", plus the save-failure case now checks
the panel stays open; Route "rests its filters as a summary, and Find route asks with an edit at
once, open or closed". Existing tests that used the controls open CHANGE first. The Route test's
fake now keeps what it saves, as the core does.

Captures at 1920 x 1080: `docs/design/collapsed-filters-sell-before.png`,
`collapsed-filters-sell-after.png` and `collapsed-filters-sell-open-after.png`. Guide updated in
`docs/guide/places.md` (How searches work) and `docs/guide/trade.md`.

Checks: `pnpm check` and `pnpm lint` clean. `pnpm test`: the node project passes (129); the
browser project collided with another worktree's Vitest on the fixed browser port 63315, so the
full suite ran as `pnpm exec vitest run --browser.api.port 63417`: 77 files, 568 tests pass.

### Responses

1-l9pq.1: fixed in 9a4f6133
1-l9pq.2: fixed in 9a4f6133
1-l9pq.3: fixed in 9a4f6133
2-72am.4: fixed in 21321e97
