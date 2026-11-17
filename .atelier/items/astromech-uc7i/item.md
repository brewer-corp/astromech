---
schema: atelier.item
schema_version: 3
id: astromech-uc7i
title: 'Route sidebar: the route is part of the frame on every page'
status: building
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

- [ ] While a route is followed and the window is at or above the dock width, a left sidebar runs from the header's rule to the status bar on every page: Hull surface, flush to the window's left edge, headed ROUTE with a collapse chevron, joined to a Hull status bar with no rule between them. It holds what the Route page holds: the plotter title (from, the destination editable in place, Plotter and Worked out, Replot from here as a text action, ? and the menu) and the timeline with the next stop raised and the commander's row marked.
- [ ] At 320 px (20rem) the timeline follows the spike: the star icon carries the star type with a Tip, names truncate with a Tip except the next stop's, which wraps; fuel and neutron notes and an inhabited stop's allegiance, security and faction state sit on a second line.
- [ ] Collapsed, and whenever the window is below the dock width, the frame shows the route strip with a chevron; below the dock width the chevron opens the same sidebar over the page, the page dimmed behind it, closing on Esc, on the chevron and on a click outside. The collapsed choice persists.
- [ ] No route: a 48 px rail with two actions, one that takes focus to Find or plot and one that opens the Import a route dialog. It reserves no more width. Below the dock width the strip's No route state offers the same two. Import stays reachable with no route at every window size, as it is today from the Route page's menu; a browser test imports a first route from the empty state, docked and below the dock width.
- [ ] The dock width is measured, not assumed: the narrowest window at which List with detail pages (the Sell finder with a result selected) keep their rows clear of their detail panel beside the sidebar, at every Text size step. The spike measured about 1,700 px under the old zoom.
- [ ] The `win` container moves from the frame to the page column, so each page's narrow rules measure the page; the ten page files that use it are rechecked with the sidebar open and closed at their switching widths. The frame's own header rule keeps measuring the window.
- [ ] The Route page is gone. Inside the app, everything that used to open the Route page (the strip's link, the palette's entries, a callout's link) opens the sidebar where the commander is and changes no page. A `#/route` address opened cold, from a bookmark or an old link, lands on the default page with the sidebar open, or the overlay below the dock width; `MOVED` only rewrites paths, so the item adds what opens the sidebar. The guide's links are updated. GALAXY becomes PLACES, one page with no page tab row; this item owns that rename, and astromech-5a01 follows it.
- [ ] The sidebar's width is the frame's, set in `App.svelte` or a frame component, not in a route file. The route files that are now frame (`Title.svelte`, `PlotSetup.svelte`, `Timeline.svelte`, `NeutronFile.svelte` and what replaces `View.svelte`) move from `PAGE_SIZING` to `PAGE_SIZING_EXEMPT` in `app/eslint.config.js` with the reason; they still use the type roles. `ImportDialog.svelte` stays enrolled.
- [ ] One poll feeds the strip and the sidebar. A failed clipboard write reads as the shared Failure line in both (this takes in astromech-rpdc). The deferred finding astromech-qwvs/1-mga3.3 (a known dry leg keeps its warning when it expands) is checked in the sidebar and fixed if it still fails.
- [ ] The route journeys (`app/journeys/route.journey.ts`), repaired by astromech-5lh2, are moved to the sidebar and pass, with the rest of the journeys.
- [ ] Browser tests cover: docked, collapsed, no route, the overlay opening and closing below the dock width, and a route appearing and ending. Captures at 1100 x 700, 1920 x 1080 and 2560 x 1440 in `docs/design/`. `docs/guide/routes.md` and `using-the-app.md` describe it.

## Scope

The frame, the route module and the Places top tab. The SHIPS and ENGINEERING tabs are astromech-5a01.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). Reference captures: `hier-B-sell-2560.png`, `hier-B-collapsed-2560.png`, `hier-A-over-page-1100.png`, `4-no-route-2560.png`, and `tone5-sell-2560.png` for colour. The route code is built as a page today: `route/Title.svelte` and `Timeline.svelte` lay out for page width and `View.svelte` owns the 2 s poll. Rework them in place; the spike's `RouteSide.svelte` is a static reference only.
