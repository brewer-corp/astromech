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
status: "in_progress"
title: "Journeys: plot a route from the page and follow it; a jump into the next stop copies and reads out the one after"
updated_at: "2026-11-07T23:48:08.776246133+00:00"
---

## Goal

The suite holds the two route journeys named by success criterion journeys-in-ci:
- the commander plots a route from the page and follows it while jumping;
- a jump into the next stop copies the stop after it and reads it out, and the page shows it.

Both drive the page through the harness against the real core and a committed world.

## Done when

- [x] Journey: plot and follow.
  - The world, under worlds/journeys/, has the commander in Sol in a ship whose jump range the plotter reads (`route_plot_ship`).
  - On the Route page the commander types a destination in the plot bar and plots (app/src/modules/route/PlotBar.svelte, Plot.svelte).
  - The core asks Spansh, which answers from recorded pages: the neutron route in worlds/preview/sol/net.json (queued, then pending, then the result, Sol to Colonia through PSR J1752-2806), or the exact route in worlds/every-command/net.json (Sol to Maia).
  - The page shows the route followed, with its next stop.
  - A journal jump into that stop, a world step, moves the page's next stop on once the page clock runs the route poll.
  - Mutation: the jump line naming a system off the route leaves the next stop where it was, and the journey fails.
- [x] Journey: jump into the next stop.
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

## Notes

- 2026-11-07 [decision] The plot journey's Spansh answers are recorded fresh with --record-net (worlds/journeys/plot/net.json, 7 Nov 2026), not taken from worlds/preview/sol's neutron route or every-command's exact route, whose job ids are all zeros and whose Sol to Colonia route has 3 stops.
  Reason: Decision astromech-1zhg: fixtures are recorded raw inputs, never invented; the existing route fixtures were trimmed or hand-made. The real route at the Asp's 36.48 ly is 284 jumps with 178 neutron boosts, and its first stop is still PSR J1752-2806.
  Rejected: The existing spansh-route-*.json fixtures: edited, so the journey would test against an invented answer
- 2026-11-07 [decision] EDSM's body list for the system arrived in (asked on every jump) is left out of both journey worlds; the world refuses it, as any request it doesn't hold. The Spansh biological-signals search asked at the same time is kept.
  Reason: EDSM's answer names other players (each body's discoverer: Molan Ryke, Redfoxxx), which the repository shouldn't hold; neither journey depends on it, so neither asserts every request was recorded.
  Rejected: Redacting the names by hand: an edited answer, against astromech-1zhg
- 2026-11-07 [decision] Each route journey waits on the core's clipboard observation as its barrier after the jump, then checks the page still shows the old next stop, then runs the page clock 2 s for the route poll and waits for the new one.
  Reason: world_step promises only that the journal line was handled, and the world's rule is that effects are awaited with world_wait, so the clipboard wait is what says the core has moved on; with it, the page's old stop is a fair check that only the poll moves it. A jump off the route therefore fails at that wait (the core never moved on), before the page.
- 2026-11-07 [decision] Two worlds: worlds/journeys/plot (worlds/preview/sol-nov's game folder, for the Asp's Loadout) and worlds/journeys/next-stop (worlds/jump's game folder and steps). In the next-stop journey the route is set with callWindow before the page opens, so the page loads with it.
  Reason: The plotter needs a Loadout that worlds/jump's journal lacks; the next-stop journey is world_steps.rs's case with the page added, so it keeps that world's steps.

## Report

Lines 1 and 2 hold and are ticked. Line 3 holds locally (20 runs in a row); its CI half waits on the PR's Linux job, so it is left unticked. Commit d1535ac8 on astromech-yis2, app/journeys/route.journey.ts.
- Line 1, plot and follow (worlds/journeys/plot). The commander is Jameson in an Asp Explorer docked at Abraham Lincoln in Sol, using sol-nov's game folder; the plot panel shows "Asp Explorer: 36.48 ly a jump", read from its Loadout.
  - On the Route page the journey fills "Colonia" in Find or plot, then `runFor(250)` for the debounce, and the Matches list offers Colonia.
  - Enter opens the plot panel, and Enter plots. The core asks Spansh: the system-names query, the route submission, and seven polls (six queued, then the route).
  - The panel shows "Following Sol to Colonia: 284 jumps · 22,000 ly · 178 neutron boosts", and the route's next stop is PSR J1752-2806.
  - The journal step jumps into PSR J1752-2806. The core copies Nova Aquila No 3 (clipboard observation). The page still shows PSR J1752-2806 until `runFor(2000)` runs its route poll, then shows Nova Aquila No 3.
  - Mutations, each reverted:
    - the jump line naming Alpha Centauri (off the route): the journey fails with "world_wait: nothing matching {kind: clipboard, text: Nova Aquila No 3} was observed within 30 s". The core never moved on, so it fails before the page wait (decision note);
    - `runFor(250)` removed: the wait for the Colonia option times out.
- Line 2, jump into the next stop (worlds/journeys/next-stop, worlds/jump's steps).
  - `callWindow("route_set", {name: "Trip", waypoints: [Sol, Alpha Centauri, Barnard's Star]})` copies Alpha Centauri, and the page opens with it next.
  - The jump step brings a clipboard observation "Barnard's Star" and a said observation "Next waypoint. Barnard's Star." (both awaited from the seq before the step).
  - The page keeps Alpha Centauri until `runFor(2000)`, then shows Barnard's Star.
  - Mutations, each reverted:
    - a jump into Luhman 16: "world_wait: nothing matching {kind: clipboard, text: Barnard's Star} was observed within 30 s";
    - `runFor(2000)` removed: the wait for Barnard's Star on the page times out.
- Line 3, locally: `node --test 'journeys/*.journey.ts'` in app/ runs all five tests.
  - Node 26: 20/20 in a row, 3.2-4.0 s a run by node:test (71 s for the loop).
  - Node 22.23 (`npx node@22`): 10/10, 3.6-4.5 s a run.
  - After the loops: no world core, world home or playwright-tracing folder left.
- Other checks: `pnpm -s lint` clean, `pnpm -s check` 0 errors, `pnpm -s test` 563 passed, `pnpm exec tsc -p journeys` 0, `cargo test -p astromech-core --test committed_worlds` 2 passed with the new worlds. No harness change was needed.

### Deviations
- The route answers are recorded fresh. The spec named worlds/preview/sol's neutron route and every-command's exact route, but those fixtures are edited (all-zero job ids, a 3-stop Sol to Colonia route).
  - The new recording was made with `--record-net` against Spansh's public endpoints: no key, and no commander in any request.
  - grep finds no commander, FID or key in it.
  - Its provenance is in each world's astromech.toml.
- EDSM's body lists, which name other players as discoverers, are left out of both worlds and refused there (decision note).

### Decisions
Four decision notes:
- the fresh recording;
- leaving EDSM's body lists out;
- the clipboard wait as the barrier before the page's route poll;
- the two worlds, with the route set before the page opens.

### Open items
- Line 3's CI half: the PR's Linux job.
- The edited fixtures remain: crates/astromech-data/tests/fixtures/spansh-route-*.json and spansh-exact-*.json (job ids of zeros), used by worlds/preview/sol, worlds/every-command and the crate tests. Replacing them with recordings is follow-up work for the plan's owner, out of this item's scope.
- Review is outstanding.
