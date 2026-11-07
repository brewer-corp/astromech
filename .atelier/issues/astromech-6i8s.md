---
created_at: "2026-11-07T04:50:17.476190205+00:00"
id: "astromech-6i8s"
issue_type: "decision"
labels: []
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
closed_at: "2026-11-07T04:50:39.844771865+00:00"
status: "done"
title: "One process: the core and the window in one Tauri app"
updated_at: "2026-11-07T04:50:39.844771865+00:00"
---

## Context

ARCHITECTURE §18 chose two processes for four reasons: (1) closing the window frees WebView2 while the copilot runs; (2) a window fault can't take down the kill switch, voice or input; (3) one API for every client, with no privileged shortcut for the window; (4) tao wants the main thread. In practice the shell registers 52 commands, 48 of them its own (journal scans, keybind edits, lookups, route plotting, the neutron star file), so state has two owners and reason 3 is already broken.

## Question

Should astromech stay two processes (the always-on core plus a separate Tauri window process, ARCHITECTURE §18) or become one Tauri process that holds the tray, the pipeline, voice, input and the window?

## Options

- Two processes, with the shell's commands moved into the core so the shell is only the window. Keeps process isolation and crash restart; keeps the IPC, endpoint file, token, start-the-core and allow-list plumbing.
- One Tauri process, every command behind one command layer that Tauri invoke, the control API and a headless mode all call. Removes that plumbing and the second executable; needs its own crash restart.

## Decision

One process, decided by Sumeet on 6 Nov 2026 in conversation. Reason 1 no longer holds: destroying the window ends WebView2's processes, and the windowless Tauri process costs a few tens of MB, so Start with Windows never creates a window and closing it destroys it. Reason 2 is mostly covered: WebView2's renderer already runs out of process, and Rust panics in commands are caught at the command boundary (`catch_unwind`, as callouts.rs and the EDDN outbox already do). The riskiest native code, sherpa-onnx speech, already runs in its own supervised process, `astromech-voice-worker` (ARCHITECTURE §5.10); this decision is about the core and the window, and the speech worker stays a separate supervised helper. The native code the merged process would share with the kill switch is SQLite and WebView2's host side. Reason 3 is a matter of discipline, kept by one command layer that every way in calls. Reason 4 is moot: the tray and window share tao's main-thread loop, and the pipeline, voice and the keyboard hook run on their own threads.

Consequences: crash restart moves to Windows' restart registration, or a minimal watchdog if that can't cover it. The neutron star file and scan caches live in the always-on process, so they load on first use and drop when idle. Idle memory must stay inside S11. The core keeps a headless mode for Linux, CI and the dev bridge. ARCHITECTURE §18's process model is superseded by this record.

## Close Reason

Decided by Sumeet on 6 Nov 2026: one Tauri process with one command layer, superseding ARCHITECTURE §18's two-process model; rationale and consequences in the Decision section.
