---
created_at: "2026-11-07T04:51:22.892724236+00:00"
id: "astromech-bp23"
issue_type: "feature"
labels:
- "size-l"
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-bp23
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-6siv"
  - kind: "issue"
    id: "astromech-8s6k"
  - kind: "issue"
    id: "astromech-96ci"
  - kind: "issue"
    id: "astromech-96dv"
  - kind: "issue"
    id: "astromech-i8vr"
  - kind: "issue"
    id: "astromech-k6da"
  - kind: "issue"
    id: "astromech-ndzx"
  - kind: "issue"
    id: "astromech-ravn"
  - kind: "issue"
    id: "astromech-ro7a"
  - kind: "issue"
    id: "astromech-se44"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "One command layer that every way in calls"
updated_at: "2026-11-07T07:43:20.111448146+00:00"
---

## Goal

Every command the window uses is registered in one command layer, the registry in crate `astromech-api`, and handled there or by the core's own handlers registered in it. Tauri `invoke` (through the shell's proxy for now), the core's control API and the headless mode all dispatch through that registry. The shell keeps only the window, startup and its link to the core (the proxy, the core start, the watchdog and the input listener). The single-process increment, astromech-y6d6, removes that link. This is the step both process models share (decision astromech-6i8s).

## Done when

- [x] Every child is done.
- [x] The shell's 52 commands are gone from `generate_handler` except one proxy. The 48 shell commands, `open_external` and `shell_info` are command-layer handlers. `core_call` and `core_status` are replaced by the proxy. The core's control commands are registry entries. Every command runs behind `catch_unwind` on a worker thread.
- [x] Every command's request and reply types are exported to TypeScript with ts-rs, and the window's transport and fake handlers are typed against them.
- [x] A test lists the window's commands and calls each through a running core's control API. This advances success criterion one-command-list, which astromech-vucf completes by running the same test against the headless core and its scripted world. The typed `call()` (astromech-96dv) limits the window to the registry's `window` entries this test calls.
- [x] The neutron star file and the scan caches have one owner, the command layer's state in the core. They load on first use and drop when idle.
- [x] The Release workflow's onboarding smoke on the bp23 PR opens every top-level page of the installed app against the real core with no "not allowed", unknown-command or internal error.

## Background

The three Unknowns this increment was filed with are resolved as follows.

- State (Unknown 1: ownership and locking): one `State` in astromech-api, built once by the core and shared by every caller through `Arc`. It reads folders through a provider the core implements over its live config, and drops what was built for a folder that changed. It holds the keybinds API, the scan caches, the remote-data client, the star map, the neutron file and the one-at-a-time flags. Each piece keeps its own short-held mutex, as in window.rs today, and recovers from poisoning instead of answering "busy". Long jobs (route search, plot, neutron download) are one-at-a-time flags, not held locks, so several clients don't block each other. The star map opens lazily in a holder, and the session asks the holder per lookup. The session uses the same client and star map (one client per process, crates/astromech-data/src/lib.rs:14-15).
- Exposure and policy (Unknown 2): decision record astromech-unh9.
- Crate and wire (Unknown 3): the registry and the control API live in `astromech-api`, which never depends on astromech-core, -input or -policy. The core's Core-bound handlers stay in astromech-core and are registered into it. The control API stays JSON lines over TCP: request `{token, cmd, args?, text?, route?, origin?}`, reply zero or more `{"progress": V}` lines then `{"ok": true, "value": V}` or `{"ok": false, "error": "…"}`. The window's calls have no read timeout, because plots poll Spansh for up to 180 s; a dropped connection is the failure. A WebSocket API waits for a browser client that connects directly, and none is in this mission.
- Migration: expand and contract on bp23's branch. The registry comes first, then two groups of moves, each leaving a one-line forwarder in the shell so the window is unchanged. After astromech-7rcn (the transport) merges, one proxy replaces the forwarders, `core_call` and `core_status`. Every child leaves the branch green.
