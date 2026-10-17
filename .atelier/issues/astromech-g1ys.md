---
created_at: "2026-10-17T17:29:23.022110042+00:00"
id: "astromech-g1ys"
issue_type: "bug"
labels:
- "alpha"
- "docs-found"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Health check misses seven actions that voice presses"
updated_at: "2026-10-17T17:41:20.312230602+00:00"
---

## Goal

The binds health rule "No keyboard key" checks only VOICE_ACTIONS (crates/astromech-keybinds/src/catalog.rs): gear, lights, hardpoints, scoop, night vision, flight assist, silent running, HUD mode, left and right panel, throttle zero. Voice also presses GalaxyMapOpen, SystemMapOpen, UI_Back, FocusCommsPanel, FocusRadarPanel, Handbrake and ToggleDriveAssist (astromech-tools Screen and Toggle), so a missing keyboard key for these is not flagged in All keybinds or in setup; the player only finds out when the voice reply says so.

## Done when

Every action astromech-tools can press is in the health rule, ideally derived from the tools so they cannot drift, with a test. Found while writing the user guide (docs/guide/controls.md says so; update it when fixed).
