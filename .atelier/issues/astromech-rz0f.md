---
created_at: "2026-11-05T06:19:44.374075993+00:00"
id: "astromech-rz0f"
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
closed_at: "2026-11-05T07:29:12.833060410+00:00"
status: "done"
title: "Add stop and Set destination on every lookup result"
updated_at: "2026-11-05T07:29:12.833060410+00:00"
---

## Goal

Every lookup result gets two actions, Add stop and Set destination, in place of today's Go, which silently replaces the followed route (decision astromech-bi94). A commander who finds Interstellar Factors mid-route can add it as a stop and carry on, or make it the destination knowing the old route can be resumed. This is the first user outcome of the galaxy rework end to end: a search result lands on the persistent route, through the core.

## Done when

- The core's route follower takes a new control command (e.g. `route_add {system, label}`) that inserts the stop as the next waypoint, merges it with a neighbour in the same system as `Route::new` does, copies it to the clipboard, and with no route (or a finished one) starts a one-stop route. Rust tests in `crates/astromech-core/src/route.rs` and the control API test in `app.rs` cover each case and fail without the change.
- Set destination replaces the route and parks the one it replaced, so the existing Resume brings it back (decision astromech-g8d5, item 3). This is a change to `Desk::set`, not a new command: today `set` forgets the parked route (`route.rs`, `st.0.parked = None`) and only `plotted` parks. After the change `set` parks the route it replaces when that route is unfinished (replacing any older parked one, so one route stays parked), and every caller inherits it: Set destination, the Route page's import, the engineer trip (`modules/engineering/Build.svelte`) and the plotter (`route_plot`). `followedNote` in `lib/route.ts` says "Replaced <name>" with a RESUME action wherever it is shown (results, the engineer trip, the plot panel). Rust tests for parking on `set` (unfinished route parked, finished route not, older parked one replaced) and the existing parking tests still pass.
- `app/src/lib/ui/SearchKit.svelte` rows show the add stop and set destination icon buttons from astromech-pey5 (tooltip and accessible label naming the place: "Add Diaguandri as a stop"); the Go button is gone. Every SearchKit lookup gets them. `fakecore.ts` handles the new command; `SearchKit.test.ts` checks both actions call the core with the row's system and label.
- `pnpm lint && pnpm check && pnpm build && pnpm test` and `cargo test -p astromech-core` pass; a `scripts/ui` screenshot of Services results with a route followed, before and after Add stop, shows the new next stop on the Route page.
- docs/guide/routes.md (and the lookup pages that mention Go) describe the two actions.

## Scope

Core route follower, its control command, `lib/route.ts`, `fakecore.ts`, SearchKit's action column. Not the row layout (astromech-jp36) and not the route strip.

## Context

- DESIGN.md: Icons (add stop, set destination), The Icon Or Word Rule, Lookup result row item 4.
- Route follower: `crates/astromech-core/src/route.rs` (`Desk::set`, `resume`, parking), `app.rs` `route_cmd`; docs/PLOTTING.md.
- `followRoute` callers: SearchKit and `modules/engineering/Build.svelte` (engineer trip: its call is unchanged, it gains the park and the "Replaced" note through `set`).
- Add stop inserts as the next stop (decision astromech-g8d5, item 2); leg-aware insertion comes with Find along route (astromech-wla4).

## Close Reason

Implementer report. Add stop and Set destination replace Go on every SearchKit result (icon buttons labelled "Add <system> as a stop" / "Set <system> as the destination"). New core command route_add {system, label?, source?} (Desk::add): inserts as the next waypoint, merges with the next or just-passed waypoint in the same system, counts a stop in the current system as reached, copies to the clipboard, and starts a one-stop route when there is no unfinished route of our own (none, finished, galaxy map). Desk::set now parks the unfinished route of our own it replaces (one parked route); finished and galaxy-map routes are not parked, and an already parked route then stays. Set destination, the engineer trip, the plotter and imports inherit this; FollowNote.svelte shows "Replaced X." with RESUME under results, the engineer trip and the plot panel. fakecore mirrors the core; guide routes.md and galaxy.md and docs/PLOTTING.md updated. Deviation: one existing assertion changed (set over a galaxy-map route keeps the parked route instead of dropping it), so a paused trip is not dropped silently (bi94). Proof: 7 new/changed Rust tests fail without the change; cargo test -p astromech-core 211 lib pass, gear_down 21 pass; clippy -D warnings and fmt clean; app lint, svelte-check 0/0, build, 398 tests; check-guide 0 problems; screenshots docs/design/route-actions-{1..5}-*.png. Not run locally: the Tauri shell crate (CI). Review: reviewer-fable-rz0f approved (astromech-fwfv); findings n0t1 and f1fo deferred to astromech-orha, jgyk to astromech-jp36. Decisions (reviewer, reversible, all matching the branch): keep the parked route when Set destination replaces a galaxy-map route; Add stop on a galaxy-map route starts a one-stop route of ours; Resume drops the current route rather than swapping; a search route is named after its system with source "search"; the Replaced note may name an earlier-parked route for now (astromech-orha).
