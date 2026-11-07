---
created_at: "2026-11-07T04:51:23.056349964+00:00"
id: "astromech-y6d6"
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
title: "One process: the core and the window in one Tauri app"
updated_at: "2026-11-07T04:51:23.056349964+00:00"
---

## Goal

astromech ships as one Tauri process holding the tray, pipeline, voice, input and the window (decision astromech-6i8s), with the second executable, endpoint file and start-the-core plumbing gone.

## Done when

- [ ] A spike, first, shows the tray working with no window and through a window crash, and that destroying the window ends WebView2's processes on Windows.
- [ ] Start with Windows creates no window; closing the window destroys it; a second start focuses or opens it at `--route`.
- [ ] Crash restart works without the shell (Windows restart registration or a minimal watchdog), shown by a test that kills the process.
- [ ] Installer, updater and Start with Windows move to the one executable; the release build and its smoke run pass.
- [ ] ARCHITECTURE.md describes one process and links astromech-6i8s.
- [ ] After merge (Sumeet's PC, with Elite running): the S11 harness shows idle memory inside budget and no WebView2 process left with the window closed (success criterion one-process). Stand-in: the windowless process's memory and process list on Windows CI.

## Unknowns

- Whether tao's event loop can own the tray with no window on every path the core uses today (tray.rs).
- What the windowless Tauri process costs at idle.
