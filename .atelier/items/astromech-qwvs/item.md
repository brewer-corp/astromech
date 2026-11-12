---
schema: atelier.item
schema_version: 3
id: astromech-qwvs
title: 'Route page: the timeline'
status: in_review
mission: astromech-xajc
labels:
- size-m
priority: high
blocked_by:
- astromech-dsyu
- astromech-k6pl
- astromech-mwh5
- astromech-sixj
- astromech-woqv
- astromech-z5oh
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A5e78e1b5-3f1e-4b4d-b3ed-888d41e195f4
- 01a1cf5f-ca4c-7342-a8e0-fddadad8e1af
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A5e78e1b5-3f1e-4b4d-b3ed-888d41e195f4
ready:
  by: conversion
  at: 2026-11-10T06:03:06.734551139Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:11.378267755Z
---

## Goal

The route shows as the agreed timeline (astromech-ok29's Design), on the Glow-up foundation: star-type icons as nodes, the leg being flown one row a jump with your position, the next stop raised with its copy icon, inhabited stops' second line, Make next on hover. Skip and Back buttons and the 'On your clipboard' line go.

## Done when

- [x] The leg being flown opens out one row a jump with each star's icon and your row marked whenever the route data lists its jumps (from NavRoute once it is plotted in the galaxy map, or from an exact route's own list). When it lists none it shows its jump count and one line saying the stars appear once the leg is plotted in the galaxy map. A jump whose star is unknown gets the unknown-star icon.
- [x] A leg ahead shows the stars passed only when the route data lists its jumps (exact routes); otherwise it shows its jump count. The 'nowhere to scoop' warning appears only on a leg where every jump is listed and every one is known not to be scoopable; a leg with a missing jump or an unknown star never claims there is no scoop.
- [x] The page is built on a layout component and the type roles, sets no width or font size of its own, and its files pass both parts of the page-sizing check; star icons come from Glyph.svelte.
- [x] The page matches the design in the preview at 2560 x 1440, and at 1100 x 700 with nothing clipped.
- [x] Copy turns to a check; a failed copy is the red error; Make next works; the stars' tooltips use Tip.
- [x] Route tests updated; docs/guide's route page describes it.

## Context

Mission astromech-xajc. The data contract is astromech-woqv's: jumps are present for a leg only when a source lists them.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-ok29 (Route page redesign: the title is the plotter, the route a timeline)

## Report

The Route page is now the timeline (`app/src/modules/route/Timeline.svelte`), inside `View.svelte` on the Board layout.

- **Flown leg.** One row a jump, each with its star icon and type, whenever the stop's `hops` are listed (NavRoute or an exact route). Your row has the map pin and "You are here" from `view.position`. With position 0, or no listed jumps but `here` in the previous stop, that stop is marked instead. When no jumps are listed: the jump count and "The stars appear once the leg is plotted in the galaxy map." A one-jump leg has no leg row. Unknown stars get `star-unknown`.
- **Legs ahead and behind.** The stars passed when listed (count added past 5, as in the mockup), else the jump count. `noScoop` in `lib/route.ts` warns only when all jumps are listed (`hops.length` equals `jumps`), there are two or more, and every one is `scoopable: "no"`. One-jump legs never warn, or every neutron stop on an exact route would.
- **Next stop.** Raised on Hull, Title role in Orange Lit, a 24 px icon, "Next stop · N jumps away" where known. Copy goes through `route_copy` and turns to the `copied` check for 1.5 s; `clipboard_error` shows as the red Failure in that row. Other stops copy with `navigator.clipboard` (as SystemChip does), with the same check or a red Failure in their row.
- **Make next** needed a core command: `route_go {index, system}` (`Desk::go_to`), refused when that stop is no longer there, so a route changed between polls is never moved to the wrong stop. It is offered on every stop but the next, ahead or behind, so it also covers Back. Not offered on galaxy-map routes.
- **Details line.** Nothing called `route_details`, so the second line would never appear outside the preview. View now calls it on open and whenever the route's stops change.
- Star tooltips use `tip()` (rich: system, type, jump N of M, note). The star mapping (`starKind`) is in `lib/route.ts`. I added a `here` map-pin glyph to Glyph.svelte and named it in DESIGN.md Icons.
- **Sizing.** Both page-sizing parts are on for `src/modules/route/{View,Timeline}.svelte` and pass. No widths or font sizes; text uses the type-role classes.
- **Tests.** View.browser.test.ts is rewritten: Make next ahead and behind, a failed `route_go`, the clipboard failure and retry with the check, another stop's copy success and failure, the fixture's flown leg, unlisted legs, and the warning rules. staleMocked uses Clear route instead of Skip. There is a unit test for `starKind`, Rust tests for `go_to`, and core-replies plus every_window_command cover `route_go`.
- **Docs.** docs/guide/routes.md describes the page; PLOTTING.md lists `route_go`.

Checks run: `pnpm lint`, `pnpm check`, `pnpm test` (617 pass), `cargo clippy -p astromech-core --all-targets -D warnings`, `cargo test -p astromech-core --lib`, `cargo test -p astromech-app --test every_window_command`, `scripts/check-guide.py`. I compared captures of `?core=ready&route=timeline` at 2560 x 1440 and 1100 x 700 (no horizontal overflow) and `route=exact` against the mockup.

Choices a reviewer may question:
- **Copy icons.** The item says copy and Make next appear on hover, but DESIGN.md rev 4 says row icons stay visible at rest and copy stays beside every system name. I followed DESIGN.md: copy is visible in Orange Mid. Make next (a word) shows on hover or focus. It is hidden with opacity, not visibility, so Tab still reaches it.
- **Left as they were.** The title, the import box, the galaxy-map help and Clear route are astromech-irrv's to rework. I only moved them onto type roles.
- **Not built.** The mockup's legend was left out (Tips name every icon, per "a page that says less"). The "here" row shares the next stop's Hull fill, since DESIGN.md bans coloured side borders, so the pin and white text set it apart.

### Round 2

All three findings of review 1 are fixed, including the deferred one:

1. **First waypoint.** The flown leg before waypoint 0 now opens out. Waypoint 0 gets a leg while it is next.
2. **Replot partway.** Position 0 marks the stop before only when `here` is that stop. Otherwise (plotted again partway along) the leg starts with a row for `here`, holding the pin and "You are here", and Sol stays unmarked.
3. **Flown-leg warning.** A known dry flown leg keeps its "No scoopable star" warning, in a row above its jumps.

The tests use real core replies: `worlds/preview/route` gains a replot from Alrai after the two jumps (mark `replotted`), and fixtures/route.json gains `/replotted` (`route` there) and `/first` (a `route_set` to PSR J1752-2806 and Colonia there), recorded with `ASTROMECH_UPDATE_FIXTURES=1`. The preview takes them as `?route=replotted` and `?route=first`. The new browser tests cover both cases and the flown-leg warning. `pnpm test` passes 619 tests, and `preview_fixtures` passes.


### Responses

1-mga3.1: fixed in 1b8c5af4
1-mga3.2: fixed in 1b8c5af4
1-mga3.3: fixed in 1b8c5af4
