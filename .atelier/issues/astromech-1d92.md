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
status: "todo"
title: "Route strip on every page"
updated_at: "2026-11-05T06:19:44.543783724+00:00"
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
