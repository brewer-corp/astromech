---
schema: atelier.item
schema_version: 3
id: astromech-uc7i
title: 'Route sidebar: the route is part of the frame on every page'
status: approved
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-3kf5
implementers:
- thread:delegated-task:command%3Amcp%3Affec3634-009b-40bd-8764-e19cb778e43b%3Adelegate-task%3Af8dab193-b90c-41d2-8512-62543b28046a
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.313028310Z
---

## Goal

The route is with the commander on every page, as part of the frame, and no longer a page of its own that uses a third of its width. Sumeet, 16 Nov 2026: the Route page does not deserve a full page; he chose a persistent sidebar and, of the two mockups, Variant B.

## Done when

- [x] While a route is followed and the window is at or above the dock width, a left sidebar runs from the header's rule to the status bar on every page: Hull surface, flush to the window's left edge, headed ROUTE with a collapse chevron, joined to a Hull status bar with no rule between them. It holds what the Route page holds: the plotter title (from, the destination editable in place, Plotter and Worked out, Replot from here as a text action, ? and the menu) and the timeline with the next stop raised and the commander's row marked.
- [x] At 320 px (20rem) the timeline follows the spike: the star icon carries the star type with a Tip, names truncate with a Tip except the next stop's, which wraps; fuel and neutron notes and an inhabited stop's allegiance, security and faction state sit on a second line.
- [x] Collapsed, and whenever the window is below the dock width, the frame shows the route strip with a chevron; below the dock width the chevron opens the same sidebar over the page, the page dimmed behind it, closing on Esc, on the chevron and on a click outside. The collapsed choice persists.
- [x] No route: a 48 px rail with two actions, one that takes focus to Find or plot and one that opens the Import a route dialog. It reserves no more width. Below the dock width the strip's No route state offers the same two. Import stays reachable with no route at every window size, as it is today from the Route page's menu; a browser test imports a first route from the empty state, docked and below the dock width.
- [x] The dock width is measured, not assumed: the narrowest window at which List with detail pages (the Sell finder with a result selected) keep their rows clear of their detail panel beside the sidebar, at every Text size step. The spike measured about 1,700 px under the old zoom.
- [x] The `win` container moves from the frame to the page column, so each page's narrow rules measure the page; the ten page files that use it are rechecked with the sidebar open and closed at their switching widths. The frame's own header rule keeps measuring the window.
- [x] The Route page is gone. Inside the app, everything that used to open the Route page (the strip's link, the palette's entries, a callout's link) opens the sidebar where the commander is and changes no page. A `#/route` address opened cold, from a bookmark or an old link, lands on the default page with the sidebar open, or the overlay below the dock width; `MOVED` only rewrites paths, so the item adds what opens the sidebar. The guide's links are updated. GALAXY becomes PLACES, one page with no page tab row; this item owns that rename, and astromech-5a01 follows it.
- [x] The sidebar's width is the frame's, set in `App.svelte` or a frame component, not in a route file. The route files that are now frame (`Title.svelte`, `PlotSetup.svelte`, `Timeline.svelte`, `NeutronFile.svelte` and what replaces `View.svelte`) move from `PAGE_SIZING` to `PAGE_SIZING_EXEMPT` in `app/eslint.config.js` with the reason; they still use the type roles. `ImportDialog.svelte` stays enrolled.
- [x] One poll feeds the strip and the sidebar. A failed clipboard write reads as the shared Failure line in both (this takes in astromech-rpdc). The deferred finding astromech-qwvs/1-mga3.3 (a known dry leg keeps its warning when it expands) is checked in the sidebar and fixed if it still fails.
- [x] The route journeys (`app/journeys/route.journey.ts`), repaired by astromech-5lh2, are moved to the sidebar and pass, with the rest of the journeys.
- [x] Browser tests cover: docked, collapsed, no route, the overlay opening and closing below the dock width, and a route appearing and ending. Captures at 1100 x 700, 1920 x 1080 and 2560 x 1440 in `docs/design/`. `docs/guide/routes.md` and `using-the-app.md` describe it.

## Scope

The frame, the route module and the Places top tab. The SHIPS and ENGINEERING tabs are astromech-5a01.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). Reference captures: `hier-B-sell-2560.png`, `hier-B-collapsed-2560.png`, `hier-A-over-page-1100.png`, `4-no-route-2560.png`, and `tone5-sell-2560.png` for colour. The route code is built as a page today: `route/Title.svelte` and `Timeline.svelte` lay out for page width and `View.svelte` owns the 2 s poll. Rework them in place; the spike's `RouteSide.svelte` is a static reference only.

## Report

The route is now part of the frame on every page, and the Route page is gone. At or above the dock
width a 320 px Hull sidebar runs from the header's full-width rule to the status bar. Below it, or
folded, the route strip's chevron opens the same sidebar over the dimmed page. With no route,
a 48 px rail offers Find or plot and Import a route.

**Decisions to check**

- **Dock width: 1,756 px at 100 percent (109.75rem).** It is `DOCK` in
  `app/src/lib/ui/layout/layout.ts`, measured on the Sell finder with a result selected at every
  Text size step: 1,755 px at 100 percent, falling to 1,751 at 180. A 1920 x 1080 display at
  100 percent docks, and so does a 2560 x 1440 one at 145 percent (1,765 px). The spike's figure
  was about 1,700. `App.browser.test.ts` checks both directions at every step: the panel sits
  beside the rows at `DOCK`, and below them with the page column 6 px (at 100 percent) narrower.
  astromech-3ca6 is rebuilding the Sell row right now. If the row grows, that test fails and
  `DOCK` has to be measured again.
- **A paused route with nothing followed shows the sidebar, not the rail.** That keeps Resume and
  the plotter reachable at dock width. The Route page and the strip already offered Resume in
  that state. The rail shows only when there is neither a route nor a paused one.
- **Sidebar timeline content.** Per the spike, the star type word is gone (the icon and its Tip
  carry it). Other names truncate with a Tip, and the next stop's name wraps between words, never
  at a hyphen. A stop's notes sit on a second line. The inhabited line shows allegiance, security
  and the faction state, and its Tip holds government, population and the faction. The summary
  line is "4 of 10 · 7 jumps left", next to Replot from here as a text action. The source prefix
  ("engineer trip ·") is dropped.
- **The status bar is now Hull.** It was Void with a full-width rule. Its Seam rule now starts at
  the sidebar's or rail's edge (a new `seam` prop), so the sidebar and the bar form one L.

**What changed**

- **One poll** (`route/follow.svelte.ts`) feeds the strip and the sidebar. The frame creates it
  and passes it by context, so no singleton leaks between tests. Spansh details (`route_details`)
  are asked for only while the sidebar is mounted. Before, the Route page asked when it opened;
  now no launch with a route triggers it unless the sidebar is shown.
- **`Sidebar.svelte` replaces `View.svelte`.** `Title`, `Timeline` and `PlotSetup` were reworked
  in place for 320 px, and `Picker` gained `bare` (no label, a Tip and an aria-label instead).
  Only the timeline scrolls, so the title's dropdowns are never clipped.
- **Routes into the sidebar.** `#/route` opened cold lands on the default page (still My
  controller; the Now page is astromech-coq6) with the sidebar shown: unfolded when docked, the
  overlay below the dock width. In the window, a `#/route` hash change, `go("route")` (the tray's
  `onRoute`), the palette's new Route entry, the strip's destination (now a button) and the plot
  panel's Show the route (now a button) all open the sidebar where the commander is. The page
  and its address stay.
- **The overlay closes** on Escape, on the chevron and on a pointerdown outside it. It ignores an
  Escape another control already handled (Picker, Menu, the destination's suggestions) and one
  inside a dialog.
- **GALAXY becomes PLACES**: group id `places`, one page, no tab row. The route manifest is
  deleted. `api: ["route"]` had no consumer.
- **The `win` container moved to the page column**, and the header measures itself (its rule is
  padding, not margin, so it spans the window). I rechecked the ten files' switching widths (900,
  1100 and 1200 px). Docked, the page column is at least 1,436 px, and at least 1,708 beside the
  rail, so none of them switch. Below the dock width the column is the whole window, as before.
  So every page switches at the same window width as on `main`. I captured Engineering at 1760
  docked and at 1100 with the overlay to confirm.
- **astromech-rpdc is taken in.** A failed clipboard write in the strip is the shared
  `<Failure inline>` with Try again, which runs `copyNext`. The sidebar already did this.
- **astromech-qwvs/1-mga3.3 still holds.** The existing test "shows a leg ahead's listed stars,
  and warns of nowhere to scoop..." (the flown leg keeps its warning) passes against the
  sidebar, so it needed no fix.
- **Lint.** `Title`, `PlotSetup`, `Timeline`, `NeutronFile`, `Sidebar` and `Rail` moved to
  `PAGE_SIZING_EXEMPT` with the reason. `ImportDialog` stays enrolled.
- **New tokens:** `--sidebar` (20rem), `--rail` (3rem) and `--scrim` (Void at 72 %). There is a
  new `import` glyph.

**Tests and checks**

- **New browser tests** in `App.browser.test.ts`: docked; folded and kept folded across runs;
  the overlay opening and closing, including a dropdown's own Escape; the rail and the strip's
  No route each importing a first route; a route appearing and being cleared; `#/route` cold and
  in the window; the palette's Route entry; and the dock width at every step. That file now
  imports `tokens.css`, because the frame's widths are real CSS. My first dock measurement ran
  without the tokens and was wrong.
- **Route tests updated** for the new text and markup. The strip test now asserts the shared
  Failure line for a failed clipboard write.
- **Journeys:** `route.journey.ts` opens the sidebar from the plot panel's Show the route and from
  the strip. All 7 journeys pass. I ran them against the debug core in the main checkout
  (`ASTROMECH_CORE`), built from this item's base: the item changes no Rust, and that saved a
  long Rust build on this host.
- `pnpm lint`, `pnpm check`, `pnpm build`, `check-dist.py`, `pnpm test` (646 tests) and
  `check-guide.py` all pass.

**Captures** (in `docs/design/`): `route-sidebar-sell-2560` (145 percent, docked, the Sell panel
beside the rows), `route-sidebar-collapsed-2560`, `route-sidebar-no-route-2560`,
`route-sidebar-sell-1920`, `route-sidebar-over-page-1100`, `route-sidebar-strip-1100` and
`route-sidebar-no-route-1100`. For before images, the Route page and the frame are in
`cells-route-clipboard-after.png` and `calmer-sell-after.png`.

**Docs:** `docs/guide/routes.md` (the sidebar, the strip, the overlay, the rail, and each Route
page reference), `using-the-app.md`, `places.md` (one PLACES page), and DESIGN.md's dock width.


**Round 1 (review 1-l239): both findings fixed.**

- **Finding 1, the ⋯ menu off-screen with a neutron route.** The cause was in `Menu.svelte`: it
  always opened leftward from its button. Now it measures the space to the button's left once
  drawn and opens from the button's left edge when there isn't enough (`dfe596b9`). A new browser
  test opens the menu for a neutron route, docked at 1920 and over the page at 1200, and checks
  it stays inside the window. Without the fix the test fails, with the menu at x = -119.
- **Finding 2, `#/route` and Route doing nothing at the dock width with no route.** A new `asked`
  flag on `RouteFollow`, set by `show()` when docked, makes the sidebar show with no route
  (`59959ec1`). Folding it returns to the rail without saving a folded choice. A route arriving
  clears the flag. A new browser test covers the cold link, the fold back to the rail and the
  palette's Route.

**Round 2 (review 2-3tmk), finding 3: an empty sidebar left behind once a route was cleared.**
This was fixed. `show()` now sets `asked` only when there is neither a route nor a paused one,
so asking for the sidebar over a route doesn't keep it once the route is cleared. The route
clearing test now opens `#/route` over a route, waits for it to be handled, and clears the
route. That test fails without the fix.

**Round 3 (review 3-ll87), finding 2: a cold `#/route` lost when the saved Text size undocks the
window.** This was fixed. The cold link now waits for the first read of the saved Text size
(`startScale()` settling) before it shows the sidebar, so it opens docked or as the overlay to
match the size the window ends up at. A new test holds `window_settings`, answers 115 percent at
1920 px and expects the overlay. That test fails without the fix.

**Round 4 (review 4-m383), finding 1: kept, now fixed.** A failed first read of the Text size
still counted as settled. The cold link now waits for a `startScale()` that returns true, and
decides docked or not at that moment, not through a separate effect. The test now fails the
first `window_settings`, holds the retry made on the next status poll, and answers 115
percent. That test fails without the fix.

**After approval (round 5): merged `main` (`c86255de`) for astromech-4wi1's token lint.** There were
two conflicts, both in CSS:

- **`App.svelte`'s header:** it keeps this item's `padding` (the rule spans the window) and takes
  4wi1's tokens in the gap.
- **`Timeline.svelte`:** it keeps the sidebar layout and swaps the remaining off-scale sizes for
  `--gap`. The node padding was 0.1875rem and is now `var(--gap)` (2 px less at 100 percent).

After the merge, lint (with the token rule), typecheck and all 711 tests pass.

**Round 6 (review 6-uisd), finding 1: two `--scrim` tokens.** This was fixed. `main` brought
`--scrim` for dialog backdrops, and the overlay's later `--scrim` overrode it. The overlay now
uses `--scrim-route`, which is in DESIGN.md's token map, and dialogs keep theirs.

### Responses

6-uisd.1: fixed in c36b0074
