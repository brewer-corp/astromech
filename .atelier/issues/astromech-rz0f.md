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
status: "todo"
title: "Add stop and Set destination on every lookup result"
updated_at: "2026-11-05T06:19:44.374075993+00:00"
---

## Goal

Every lookup result gets two actions, Add stop and Set destination, in place of today's Go, which silently replaces the followed route (decision astromech-bi94). A commander who finds Interstellar Factors mid-route can add it as a stop and carry on, or make it the destination knowing the old route can be resumed. This is the first user outcome of the galaxy rework end to end: a search result lands on the persistent route, through the core.

## Done when

- The core's route follower takes a new control command (e.g. `route_add {system, label}`) that inserts the stop as the next waypoint, merges it with a neighbour in the same system as `Route::new` does, copies it to the clipboard, and with no route (or a finished one) starts a one-stop route. Rust tests in `crates/astromech-core/src/route.rs` and the control API test in `app.rs` cover each case and fail without the change.
- Set destination replaces the route and parks the one it replaced, so the existing Resume brings it back; the note under the results says so ("Replaced <name>. RESUME"). Rust test for the park.
- `app/src/lib/ui/SearchKit.svelte` rows show the add stop and set destination icon buttons from astromech-pey5 (tooltip and accessible label naming the place: "Add Diaguandri as a stop"); the Go button is gone. Every SearchKit lookup gets them. `fakecore.ts` handles the new command; `SearchKit.test.ts` checks both actions call the core with the row's system and label.
- `pnpm lint && pnpm check && pnpm build && pnpm test` and `cargo test -p astromech-core` pass; a `scripts/ui` screenshot of Services results with a route followed, before and after Add stop, shows the new next stop on the Route page.
- docs/guide/routes.md (and the lookup pages that mention Go) describe the two actions.

## Scope

Core route follower, its control command, `lib/route.ts`, `fakecore.ts`, SearchKit's action column. Not the row layout (astromech-jp36) and not the route strip.

## Context

- DESIGN.md: Icons (add stop, set destination), The Icon Or Word Rule, Lookup result row item 4.
- Route follower: `crates/astromech-core/src/route.rs` (`Desk::set`, `resume`, parking), `app.rs` `route_cmd`; docs/PLOTTING.md.
- `followRoute` callers: SearchKit and `modules/engineering/Build.svelte` (engineer trip, unchanged).
