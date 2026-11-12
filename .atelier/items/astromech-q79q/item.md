---
schema: atelier.item
schema_version: 3
id: astromech-q79q
title: 'Status bar: the ship and the app''s health on every page'
status: changes_requested
mission: astromech-xajc
labels:
- size-s
- feature
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A25ad52ec-7b93-4c81-9b11-f78784707905
ready:
  by: conversion
  at: 2026-11-10T06:03:07.460001591Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:10.883768802Z
---

## Goal

The ship (name, jump range, fuel, cargo) and the app's health (Elite running, Voice ready) sit in one bar at the foot of every page, so no page owns the ship and the header has room.

## Done when

- [x] app/src/lib/ui/StatusBar.svelte is in App.svelte's frame on every page, 32 px before scaling: on the left the current ship's name and type and its unladen maximum jump range (from the engineering_fleet reply's current ship), then fuel and cargo; on the right the status line that App.svelte's header shows today (headerState), which leaves the header.
- [x] Fuel (main tank against capacity) and cargo (tonnes aboard against capacity) come from a new window command in the command layer, ship_status, read from the game state's Status and Loadout; it has a test over a recorded Status and Loadout, a fake handler, and a preview fixture dumped the usual way. With Elite not running the bar shows the ship without fuel and cargo.
- [x] The bar updates as the game writes Status (the page asks again on the same 2 s tick App.svelte already runs for status); a browser test covers ship present, Elite not running, and the command failing (the bar keeps the ship and drops the two figures; no error block).
- [x] At 1100 x 700 the frame's fixed parts (header, route strip, page tabs, status bar) leave the page at least 480 px of height; if they do not, the status bar joins the route strip's row at that size. DESIGN.md's Frame states the budget and the header's second-row breakpoint is remeasured.
- [x] Captures at 1100 x 700 and 2560 x 1440 in docs/design/; pnpm check, pnpm lint, pnpm test and cargo test for the touched crates pass.

## Context

Mission astromech-xajc. CoreStatus (the status reply) has no ship, fuel or cargo fields; fuel is in the core's game state (fuel_main) but on no window command. A new command needs a fake handler (app/src/preview/coverage.test.ts) and the fixture rules in CONTRIBUTING.md.

## Report

The ship and the app's health now sit in a 32 px status bar at the foot of every page, and the
core's state line has left the header.

- **Bar** (`app/src/lib/ui/StatusBar.svelte`, in App.svelte's frame): the current ship's name
  (Orange), type (Orange Mid) and "Jump 18.0 ly" from `engineering_fleet`'s current ship, then
  "Fuel 24.6 | 32 t" and "Cargo 120 | 256 t" as multi-value stats; on the right `headerState`,
  Orange when Elite or the core isn't running. The frame is now a flex column, so the page takes
  the rest whether or not the area has page tabs. New token `--status-bar: 32px`.
- **Command** `ship_status` (astromech-core's commands, `Done<ShipStatus>`): `fuel`,
  `fuel_capacity`, `cargo`, `cargo_capacity` from the session hub's game state (Status.json and
  the last `Loadout`), all null with no session (Elite not running). It also carries `ship_id`
  (the last `Loadout`'s), which is not in the Done when: the bar re-reads the fleet only when that
  id differs from the fleet's current ship (a ship swap), or after a failed fleet read, at most
  once per tick, instead of fetching the whole fleet every 2 s.
- **Tick**: App.svelte's 2 s status tick asks `status` and `ship_status` together.
- **Tests**: `every_window_command.rs` checks the reply against the every-command world's
  recorded Loadout and Status.json (Asp: 32 of 32 t, 0 of 64 t, ship 1; a wrong value fails it,
  checked). New preview world `worlds/preview/ship` (the engineering world's Anaconda Loadout,
  re-dated, plus a written Status.json) dumps `fixtures/ship-status.json` through
  `preview_fixtures.rs`. Fake handler in fakecore.ts: the sample under `?core=ready`, nulls in
  the other scenarios, and no core gives a failed call. `StatusBar.browser.test.ts`: Elite not
  running (ship, no fuel or cargo), then the next tick with Elite running (figures appear), and
  the call failing (ship kept, figures dropped, no error text or Failure).
  `App.browser.test.ts`'s Shipyard test now queries the "Anaconda" option by role, since the bar
  also shows "Anaconda".
- **Height budget** at 1100 x 700, measured in the preview: header 108 (stacked) + strip 36 +
  page tabs 44 + bar 32 leave main exactly 480 px, so the bar keeps its own row. DESIGN.md's Frame
  has the Status bar row, the budget and a new Status bar component section; the token map lists
  `--status-bar`.
- **Header breakpoint remeasured** (fonts loaded, binary search on the `stacked` class): one row
  from about 1456 px (was about 1565 with the usual status), and about 1606 px with Finish setup
  (was about 1900).
- **Captures**: `docs/design/status-bar-1100.png`, `docs/design/status-bar-2560.png`
  (`?core=ready#/materials`). They show fuel and cargo beside "Waiting for Elite": the fake's
  `status` always reports Elite not running while its ready scenario answers session commands
  (the same is true of `exobiology`); the real core shows figures only in a session.
- **Checks**: pnpm check, pnpm lint, pnpm test (565 passed), cargo test -p astromech-core
  -p astromech-app, cargo clippy on both with -D warnings, cargo fmt --check: all pass.
- Height scaling (astromech-k6pl) is not done yet; the bar uses the token, so it scales with the
  rest once that lands.

### Responses

1-77tl.1: fixed in 8474debf: ship_status also carries the last Loadout's ship_name and max_jump_range, and the bar reads engineering_fleet again once whenever ship_id, name or range changes; StatusBar.browser.test.ts covers a same-ID range change (18.0 to 21.4 ly).
1-77tl.2: fixed in 8474debf: docs/guide/using-the-app.md describes the status bar (ship, jump range, fuel and cargo, when they drop out, the 2 s refresh) and the health line in it, and gives the remeasured header widths.
