---
schema: atelier.item
schema_version: 3
id: astromech-woqv
title: Route data for the timeline
status: ready
mission: astromech-xajc
labels:
- size-m
priority: high
blocked_by: []
implementers: []
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

- [ ] The route view gives each stop its system and, where a source knows it, its star class; and each leg its jump count and distance. A leg's jumps (system, star class, scoopable) are present only when a source lists them, and are otherwise absent, never guessed: the contract per source is written in the reply type's doc comment and docs/ARCHITECTURE.md.
- [ ] Source by source, with a recorded example each in the tests: local neutron and Spansh neutron (stops, each leg's jump count and distance, no jumps); Spansh exact (every jump with its system and scoopable or neutron flag, star class unknown unless NavRoute supplies it); imported CSV (what its columns give).
- [ ] A leg's jumps come from the first source that has them, in this order: NavRoute.json, for the leg being flown, when the route the commander plotted in the galaxy map ends at the next stop (systems and star classes; scoopable derived as crates/astromech-announce/src/fuel.rs does); then the plotter's own list (exact routes, for every leg); then none, and the leg is count-only. Each jump's scoopable is yes, no or unknown, and unknown when its star class is missing (RouteHop.star_class is optional) and the plotter gave no flag. The view says the commander's position in the flown leg from the journal.
- [ ] Inhabited stops' details (allegiance, government, security, population, controlling faction and state) come from Spansh, fetched once per stop and cached; a failed lookup leaves the stop without them.
- [ ] The route stores its plot settings (destination, plotter, engine, efficiency, range) and route_replot plots again from the current system with them; a route saved before this change still loads, without replot settings.
- [ ] The preview fixture has a route with a flown leg that has jumps, a later leg with only a count, and an inhabited stop; the fixture drift test passes.

## Context

Mission astromech-xajc, parent astromech-ok29. The first version of this item promised every jump's system and class from the plotters; the planning review found they do not supply them (review finding astromech-50kz).

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-ok29 (Route page redesign: the title is the plotter, the route a timeline)
