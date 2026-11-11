---
schema: atelier.item
schema_version: 3
id: astromech-g0be
title: Presses reach the page through the command layer, so set-up-push-to-talk and keybind journeys run in world mode
status: draft
mission: null
labels:
- size-m
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T22:55:23.422732958Z
---

## Goal

The page's press capture works against the headless world core through the dev bridge. The page uses it to set up push-to-talk and the kill switch, and to press-to-bind a keybind. Once it works, the two journeys astromech-yis2 left out can run:
- set up push-to-talk;
- edit a keybind, and the right .binds file changes.

Today the page gets presses only from the shell's "input" event:
- the page side is app/src/lib/binds.ts:41-43 and app/src/lib/store.svelte.ts:52;
- the shell emits it from its Windows-only press listener (app/src-tauri/src/host.rs:259-300 on astromech-y6d6's branch);
- the bridge never carries it (app/src/lib/transport.ts:16-18, :93-97);
- the world core's fake raw input feeds the session's controls, not that listener.

## Done when

- [ ] The window's presses come from the core through the command layer, the same way in the app and through the dev bridge. The shell no longer has a press listener of its own.
- [ ] In world mode, a world `press` step reaches the page as a press.
- [ ] Journey: in onboarding's buttons step (app/src/lib/onboarding/StepButtons.svelte), pressing the world's button sets push-to-talk, and `status` shows it.
- [ ] Journey: press-to-bind a keybind (app/src/lib/ui/BindDialog.svelte) changes only the active preset's .binds file in the world's bindings folder, in that one slot.

## Context

- **Wait for astromech-y6d6.** It owns the shell's press listener and the single Raw Input registration (astromech-7i2k).
- **Why this is filed now.** astromech-yis2's planning found this gap on 7 Nov 2026. The journeys that need presses were left out of yis2 for it.
- **The mission.** This item isn't needed for mission astromech-kjfs's success criteria. Pick it up after y6d6 merges, or when a press-capture bug shows up.

## Unknowns

- Whether a long-lived streaming command fits the control wire (one request per connection, crates/astromech-api/src/control.rs), or the window needs a subscription. This is a command-layer contract (decision astromech-unh9's area), so it gets a decision record when this item is picked up.
