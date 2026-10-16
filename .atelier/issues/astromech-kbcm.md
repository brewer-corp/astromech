---
created_at: "2026-10-16T18:12:14.965390748+00:00"
id: "astromech-kbcm"
issue_type: "bug"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-snqt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "All keybinds: group panels expand together"
updated_at: "2026-10-16T18:12:14.965390748+00:00"
---

## Goal

On the All keybinds page, expanding one group panel expands every panel. Each panel should open and close on its own.

## Done when

A component test opens one group and shows the others stay closed, and the test fails on the current code.

## Context

`app/src/modules/keybinds/KeybindsView.svelte`: `open` state and `isOpen()` (around lines 18, 49 and 66). Reproduce the bug before changing anything.
