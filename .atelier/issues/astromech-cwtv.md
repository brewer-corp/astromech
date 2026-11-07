---
created_at: "2026-11-07T22:55:23.422993154+00:00"
id: "astromech-cwtv"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journeys: plot a route from the page and follow it; a jump into the next stop copies and reads out the one after"
updated_at: "2026-11-07T22:55:23.422993154+00:00"
---

## Goal

The suite holds the two route journeys named by success criterion journeys-in-ci:
- the commander plots a route from the page and follows it while jumping;
- a jump into the next stop copies the stop after it and reads it out, and the page shows it.

Both drive the page through the harness against the real core and a committed world.

## Done when

- [ ] Journey: plot and follow.
  - The world, under worlds/journeys/, has the commander in Sol in a ship whose jump range the plotter reads (`route_plot_ship`).
  - On the Route page the commander types a destination in the plot bar and plots (app/src/modules/route/PlotBar.svelte, Plot.svelte).
  - The core asks Spansh, which answers from recorded pages: the neutron route in worlds/preview/sol/net.json (queued, then pending, then the result, Sol to Colonia through PSR J1752-2806), or the exact route in worlds/every-command/net.json (Sol to Maia).
  - The page shows the route followed, with its next stop.
  - A journal jump into that stop, a world step, moves the page's next stop on once the page clock runs the route poll.
  - Mutation: the jump line naming a system off the route leaves the next stop where it was, and the journey fails.
- [ ] Journey: jump into the next stop.
  - astromech's own route is followed: Sol, then two more stops. It is set up through `callWindow("route_set", …)`, as crates/astromech-core/tests/world_steps.rs:28-38 does, since plotting is the journey above.
  - A journal jump into the next stop makes the core:
    - copy the stop after it (a `clipboard` observation);
    - say "Next waypoint. <system>." (a `said` observation).
  - The page's route shows that stop as next.
  - Mutation: a jump to a system off the route fails the clipboard wait.
- [ ] Both pass in the journey suite (`node --test 'journeys/*.journey.ts'` in app/) locally, 20 runs in a row with no failure, and in CI's Linux job.

## Scope

In: the two journeys, their worlds and any harness helper they need.

Out: the route desk's own cases, which route.rs's tests cover: legs, detours, parking and copy failures. The journeys don't repeat them.

## Context

- **Order.** Start once astromech-8qsx (the journey harness) is on yis2's branch.
- **Which route copies.** A route plotted in the galaxy map is followed silently, without the clipboard (crates/astromech-core/src/route.rs, `a_detour_in_the_galaxy_map_parks_the_route` and the module docs). So the copy journey needs a route of astromech's own.
- **Rust coverage.** worlds/jump and world_steps.rs:55-66 already cover the core's half, the copy and the read-out, from Rust. The journey adds the page.
- **Journal lines.** They are hand-built in the shape the game writes, never personal logs (AGENTS.md, Constraints).
- **Spansh polling.** In world mode Spansh jobs are polled with no wait (world.rs:655-660). A recording that leaves a job queued fails the lookup after 100 polls.
