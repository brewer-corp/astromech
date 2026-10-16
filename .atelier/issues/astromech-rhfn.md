---
created_at: "2026-10-16T18:56:53.793019486+00:00"
id: "astromech-rhfn"
issue_type: "bug"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "A mouse click on All keybinds opens every group"
updated_at: "2026-10-16T18:56:53.816922649+00:00"
---

## Goal

In the app window, clicking a group header on All keybinds opens every group. Raw Input forwards mouse buttons as `{device:"Mouse", key:"Mouse_1"}`; when Mouse_1 is bound (on-foot Fire, and in the game defaults) the press-to-find effect sees a hit, opens every untoggled group, clears the search and selects the action. astromech-kbcm fixed a different path (three or fewer groups all opening).

## Done when

A component test clicks a group header and injects a Mouse_1 press, and only that group is open; the test fails on the code before the fix.

## Context

`app/src/modules/keybinds/KeybindsView.svelte` (`isOpen` default `hits.length > 0`, press effect), `app/src-tauri/src/rawinput.rs`, `tracker.rs`. Found in review of astromech-q318 (round 1, R1). Not reproduced on Windows; inferred from the input code and an injected-press test.
