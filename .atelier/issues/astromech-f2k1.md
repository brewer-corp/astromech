---
created_at: "2026-10-22T21:50:43.743780411+00:00"
id: "astromech-f2k1"
issue_type: "bug"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Window's config path at startup: --config runs and a not-yet-written config file"
updated_at: "2026-10-22T21:50:43.743780411+00:00"
---

## Goal

The app window uses the same config file, data folder and Bindings folder as the core it talks to, from the moment it opens.

## Done when

Tests cover both gaps: (1) the window sets up its remote-data cache from the config the running tray core published, not from a stale config.path left by an earlier --config run whose file still exists; (2) a tray core started with --config pointing at a file that doesn't exist yet and the window agree on the folders until the core writes that file.

## Context

Review findings R2 (astromech-nhfz) and R3 (astromech-5ij6) on feature/astromech-0k88: crates/astromech-store/src/paths.rs:92 (fallback to the default file), app/src-tauri/src/main.rs:1229 (remote-data cache set up before the tray core publishes), crates/astromech-core/src/app.rs:1238.
