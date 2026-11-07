---
created_at: "2026-11-07T04:51:22.892724236+00:00"
id: "astromech-bp23"
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
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "One command layer that every way in calls"
updated_at: "2026-11-07T04:51:22.892724236+00:00"
---

## Goal

Every command the window uses lives in one Rust command layer in a crate. Tauri `invoke`, the core's control API and the headless mode all call the same handlers, and the shell keeps only the window and startup. This is the step both process models share (decision astromech-6i8s).

## Done when

- [ ] The about 42 shell commands in app/src-tauri/src/window.rs and the core's control commands are handlers in the command layer, each run behind `catch_unwind` on a worker thread.
- [ ] Their request and reply types are exported to TypeScript (tauri-specta or ts-rs), and the window's transport and fake handlers are typed against them.
- [ ] A test lists the window's commands and calls each through a running core's control API (success criterion one-command-list).
- [ ] The neutron star file and the scan caches have one owner, load on first use and drop when idle.

## Unknowns

- How the commands' state (binds API, neutron file, caches, galaxy) is owned and locked once one process serves several clients.
- Which crate holds the layer, and whether the control API moves from JSON lines to the planned WebSocket API (astromech-proto) now or later.
