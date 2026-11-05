---
created_at: "2026-11-05T06:19:44.543783724+00:00"
id: "astromech-1d92"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-pey5"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T07:57:17.230979782+00:00"
status: "done"
title: "Route strip on every page"
updated_at: "2026-11-05T07:57:17.230979782+00:00"
---

## Goal

Show the followed route as a 36 px strip under the header on every page, replacing the Galaxy-only right rail, so the next stop is always on screen (DESIGN.md Route strip; decision astromech-bi94).

## Done when

- `App.svelte` renders a route strip on every page, Onboarding excepted: "Next" · next system (Orange Lit) · copy icon · "3 of 41" · the next waypoint's flags as badges (today's waypoint labels: neutron boost, fuel) · on the right the destination and how far is left. With no route: "No route" in Orange Mid and a Find or plot shortcut that focuses the header search box. An orange "!" with plain words when the commander should act: a failed clipboard write, or the commander's current system is not the previous or next waypoint ("Off route"). Never a second line.
- "N jumps left" where the jumps are known: plotted routes record each waypoint's jumps (an optional `jumps` on the core's `Waypoint`, filled from `astromech_galaxy::plot`; `route.json` without it still loads). Routes without them (imports, lookups) say "N stops left" (decision astromech-g8d5, item 10). Rust test for the old-format load and the count.
- The Galaxy split layout and `modules/route/Rail.svelte` are gone; the Route page keeps the full list, import and Resume.
- Component tests for the strip's states (route, no route, off route, clipboard error); `pnpm lint && pnpm check && pnpm build && pnpm test` and `cargo test -p astromech-core -p astromech-galaxy` pass.
- `scripts/ui` screenshots at 1100 px and 1600 px width of three pages (Materials, Services, Settings) with a route followed, and of the no-route state. The browser preview starts with no route (`fakecore.ts` sets `route: null`), so this child adds a preview scenario that starts on a route: a URL flag read by `fakecore.ts` (e.g. `?core=ready&route=exact`) that follows `fixtures/plot.json`'s exact route with "here" at its third waypoint, so `scripts/ui` reproduces the screenshots in one command; nothing in the strip wraps at 1100 px.
- docs/guide/routes.md describes the strip.

## Scope

The frame and a new strip component in `app/src/lib/ui` or `modules/route`; the core `Waypoint` field. Not the header search box (Find or plot child).

## Context

- DESIGN.md: Layout/Frame, Route strip, Badge, States; mockups `docs/design/design-system-places-*-after.png` show the strip.
- Uses Badge and Icon button from astromech-pey5.
- `app/src/lib/route.ts` (`RouteView`, `watchRoute`), `modules/route/View.svelte`, `crates/astromech-core/src/route.rs`, `crates/astromech-galaxy/src/plot.rs` (per-leg jumps).

## Close Reason

Implementer report. Route strip on every page (36 px, under the header, not in onboarding): Next · system · copy · "n of N" · flag badges · To destination (links to #/route) · "N jumps left" for plotted and galaxy-map routes, or "N stops left" when a stop does not know its jumps (g8d5 item 10). Warnings, one at a time, orange "!": clipboard failed, Off route, Refuel before the next jump at a fuel stop; a failed core read is red. No route or finished: Find or plot, which focuses the header box (PlotBar exports focusFindOrPlot for 2hbw); a parked route shows Paused <name> + Resume once the followed one has ended. Never a second line (worst case checked at 1100 and 1600 px). Rail.svelte and the Galaxy split are gone; the Route page keeps list, import and Resume; Services/Outfitting/Shipyard keep their spacing at the 1200 px list width. Core: optional Waypoint.jumps (from astromech_galaxy::plot for exact and neutron routes, 1 per galaxy-map waypoint, cleared on the leg after an Add stop) and View.jumps_left; old route.json without jumps loads. Preview: ?core=ready&route=exact. Deviation from the Done when: Off route fires only when the next waypoint is one jump on, not whenever the commander is in neither the previous nor next waypoint (the literal rule flags every neutron leg and engineer trip); decided by the reviewer. Added the refuel warning DESIGN.md names (it cannot see the tank, so it stays until the jump). Proof: cargo test -p astromech-core -p astromech-galaxy 418 pass; clippy -D warnings, fmt clean; app lint, svelte-check 0/0, build, 408 tests (Strip.test.ts, frame test); check-guide, check-deps OK; mutations of the strip, the one-jump rule and the jump reset each fail a test; screenshots docs/design/route-strip-*.png. Not run locally: the Tauri shell build (CI). Review: reviewer-fable-1d92 approved (astromech-kk4y); w52v and aeub folded into astromech-jp36, 9ju1 and 7464 deferred to astromech-nh0g, 9e5p answered here. Reviewer decisions (reversible, matching the branch): one-jump off-route rule; keep the refuel warning, Status.json fuel later; keep the destination link; keep the "Find or plot" label; keep the 1200 px cap; clipboard failure orange, failed call red. Follow-up noted: the strip and the Route page each poll the route every 2 s.
