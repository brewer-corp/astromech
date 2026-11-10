---
created_at: "2026-11-07T03:00:10.883768802+00:00"
id: "astromech-q79q"
issue_type: "feature"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Status bar: the ship and the app's health on every page"
updated_at: "2026-11-10T06:03:07.460001591+00:00"
---

## Goal

The ship (name, jump range, fuel, cargo) and the app's health (Elite running, Voice ready) sit in one bar at the foot of every page, so no page owns the ship and the header has room.

## Done when

- [ ] app/src/lib/ui/StatusBar.svelte is in App.svelte's frame on every page, 32 px before scaling: on the left the current ship's name and type and its unladen maximum jump range (from the engineering_fleet reply's current ship), then fuel and cargo; on the right the status line that App.svelte's header shows today (headerState), which leaves the header.
- [ ] Fuel (main tank against capacity) and cargo (tonnes aboard against capacity) come from a new window command in the command layer, ship_status, read from the game state's Status and Loadout; it has a test over a recorded Status and Loadout, a fake handler, and a preview fixture dumped the usual way. With Elite not running the bar shows the ship without fuel and cargo.
- [ ] The bar updates as the game writes Status (the page asks again on the same 2 s tick App.svelte already runs for status); a browser test covers ship present, Elite not running, and the command failing (the bar keeps the ship and drops the two figures; no error block).
- [ ] At 1100 x 700 the frame's fixed parts (header, route strip, page tabs, status bar) leave the page at least 480 px of height; if they do not, the status bar joins the route strip's row at that size. DESIGN.md's Frame states the budget and the header's second-row breakpoint is remeasured.
- [ ] Captures at 1100 x 700 and 2560 x 1440 in docs/design/; pnpm check, pnpm lint, pnpm test and cargo test for the touched crates pass.

## Context

Mission astromech-xajc. CoreStatus (the status reply) has no ship, fuel or cargo fields; fuel is in the core's game state (fuel_main) but on no window command. A new command needs a fake handler (app/src/preview/coverage.test.ts) and the fixture rules in CONTRIBUTING.md.
