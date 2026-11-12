---
schema: atelier.item
schema_version: 3
id: astromech-woqv
title: Route data for the timeline
status: changes_requested
mission: astromech-xajc
labels:
- size-m
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A493ed5d0-438d-473a-ae5a-443cdf843127
ready:
  by: conversion
  at: 2026-11-10T06:03:06.371200258Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:11.045308872Z
---

## Goal

The core gives the Route page what the timeline shows, and says plainly what it does not know. The plotters return stops, not the systems between them: the local neutron plotter and Spansh's neutron plotter give each leg a jump count and a distance only (crates/astromech-galaxy/src/neutron_plot.rs, Leg), and Spansh's exact plotter gives each jump's system with scoopable and neutron flags but no star class. The game's NavRoute.json, which the game state already holds (crates/astromech-gamestate, apply_route), lists every jump of the leg the commander has plotted in the galaxy map, with its star class.

## Done when

- [x] The route view gives each stop its system and, where a source knows it, its star class; and each leg its jump count and distance. A leg's jumps (system, star class, scoopable) are present only when a source lists them, and are otherwise absent, never guessed: the contract per source is written in the reply type's doc comment and docs/ARCHITECTURE.md.
- [x] Source by source, with a recorded example each in the tests: local neutron and Spansh neutron (stops, each leg's jump count and distance, no jumps); Spansh exact (every jump with its system and scoopable or neutron flag, star class unknown unless NavRoute supplies it); imported CSV (what its columns give).
- [x] A leg's jumps come from the first source that has them, in this order: NavRoute.json, for the leg being flown, when the route the commander plotted in the galaxy map ends at the next stop (systems and star classes; scoopable derived as crates/astromech-announce/src/fuel.rs does); then the plotter's own list (exact routes, for every leg); then none, and the leg is count-only. Each jump's scoopable is yes, no or unknown, and unknown when its star class is missing (RouteHop.star_class is optional) and the plotter gave no flag. The view says the commander's position in the flown leg from the journal.
- [x] Inhabited stops' details (allegiance, government, security, population, controlling faction and state) come from Spansh, fetched once per stop and cached; a failed lookup leaves the stop without them.
- [x] The route stores its plot settings (destination, plotter, engine, efficiency, range) and route_replot plots again from the current system with them; a route saved before this change still loads, without replot settings.
- [x] The preview fixture has a route with a flown leg that has jumps, a later leg with only a count, and an inhabited stop; the fixture drift test passes.

## Context

Mission astromech-xajc, parent astromech-ok29. The first version of this item promised every jump's system and class from the plotters; the planning review found they do not supply them (review finding astromech-50kz).

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-ok29 (Route page redesign: the title is the plotter, the route a timeline)

## Report

The core's route view now carries the timeline (`View::stops`, `View::position`), with the
per-source contract on `StopView`'s doc comment and in docs/ARCHITECTURE.md ("Route data for the
timeline"). Commit 6d4b0d89.

What was built, line by line:

- Stops and legs: `Waypoint` gained `ly`, `star_class`, `scoopable`, `neutron` and `hops` (all
  optional, written only when a source gives them); the plotters fill them through
  `astromech_galaxy::plot::Stop`. The view resolves each stop and jump to a `star_class`, a
  `scoopable` of yes, no or unknown (class first, as `fuel::scoopable` does, then the plotter's
  flag), and `neutron`.
- Sources, each with a recorded example: Spansh neutron and exact in
  `app::tests::plotted_routes_load_into_the_follower` (the recorded jobs); the local neutron
  plotter in plot.json's `local` (dump-local-route example); CSV in
  `route::tests::a_csv_import_gives_what_its_columns_give`, over CSVs written from the two
  recorded jobs with the column lists in Spansh's site script (read 12 Nov 2026:
  `controllers/plotter/results` and `exact-plotter/results`).
- Order of a leg's jumps: NavRoute.json for the flown leg when it ends at the next stop, then
  the plotter's list, then none (`route::tests::the_flown_leg_lists_its_jumps_from_the_galaxy_map`,
  `a_galaxy_map_route_lists_a_leg_only_when_it_ends_at_the_next_stop`). `Follower::plotted` now
  takes `RouteHop`s, and the desk keeps the last galaxy-map route in route.json (`nav`), cleared by
  NavRouteClear.
- Details: `astromech_galaxy::stops::details` asks Spansh's systems search by name (tests/stops.rs,
  recorded answer for the Sol to Maia stops, two of them uninhabited). The new `route_details`
  command fetches stops not yet asked for, once per stop per run, and a failure leaves them
  without; the polled `route` command never touches the network.
- Replot: `Plotted::plot` (`PlotSettings`) goes to `route_set` and is kept as `Route::plot`;
  `route_replot` (astromech-api, beside `route_plot`) reads it through `route` and plots again from
  the current system. `Ask` gained optional `engine` and `range` for that. A route.json without
  settings loads (`plot_settings_are_kept_and_optional`), and replot then says so.
- Preview: new world `worlds/preview/route` and fixture `app/src/preview/fixtures/route.json`
  (Sol, PSR J1752-2806 after 10 jumps, Colonia after 1; the first leg plotted in the galaxy map
  with 11 jumps and two made, so `position` is 2; all three stops inhabited). The drift test
  passes. The fake core answers `route_details` and `route_replot`, and `?route=timeline` starts on
  the recorded view.

Choices a reviewer should check:

- Exact routes keep every jump as a stop (the follower copies each), so each exact leg lists one
  jump, the stop itself. Grouping exact jumps into legs between refuel stops would change what is
  copied and followed, which this item doesn't ask for.
- Spansh's exact plotter flags both ends not scoopable whatever the star (Sol, a G star, and Maia,
  a B star, in the recorded job), so the start and the destination keep no scoopable flag.
- `route_replot` uses the stored range, as the Done-when line says ("with them"). If the Route
  page's Replot should use the current ship's range once the "range differs" line shows, that is
  one line in `Settings::ask`.
- Details live in memory for the run, not in route.json, and a failed lookup is not retried until
  the next run, so a poll can never fan out requests.
- NavRoute.json's star classes in the world are the systems' main stars from Spansh's system
  pages, and its systems are Spansh's exact plotter's jumps for that leg (12 Nov 2026), as the
  world's astromech.toml records.

Checks run: cargo fmt, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test
--workspace` (all pass), check-deps, check-guide, and in app/ `pnpm lint`, `pnpm check`, `pnpm
build`, check-dist, `pnpm test` (563 pass), `tsc -p journeys` and the 7 journeys. Not run:
`cargo deny check` (no new dependencies; astromech-ov96 tracks its current failure).
