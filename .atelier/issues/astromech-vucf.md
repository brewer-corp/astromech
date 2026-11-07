---
created_at: "2026-11-07T04:51:23.220788707+00:00"
id: "astromech-vucf"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bp23"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A scripted world for the real core: headless mode, clock, game folder, recorded network"
updated_at: "2026-11-07T04:51:23.220788707+00:00"
---

## Goal

The real core runs headless on Linux against a scripted world: a game folder written over time (journal, Status.json, NavRoute.json and the rest), recorded Spansh and Ardent responses, a clock the test controls, and fake input, audio and clipboard, with faults as inputs. The browser preview can use it through a dev bridge, and `tauri dev` is the documented loop at the PC.

## Done when

- [ ] `astromech --headless --world <script>` (name to settle) runs the core with no window on Linux; CI builds and starts it.
- [ ] The core reads time through a clock trait, and a scripted world can advance it.
- [ ] HTTP lookups go through a record-and-replay layer that can also answer with a 503, a timeout or garbage.
- [ ] A dev bridge (Vite middleware) passes the browser's `call()` to the headless core, so `pnpm dev` with a world script shows the real core's output; documented in CONTRIBUTING.md with `tauri dev`.
- [ ] The hand-written fixtures that the real core can now produce are dumped from it, and a test fails when they drift.

## Unknowns

- Which Windows-only parts (sending keys, the keyboard hook, WASAPI audio) still lack Linux fakes.
- The world script format: reuse astromech-mockgame and the replay tests' formats, or a new one.
