---
schema: atelier.item
schema_version: 3
id: astromech-7o1g
title: The window can shrink to 800 x 500, below the 1100 x 700 the design is built for
status: building
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A85dfb9ef-882b-414a-965f-0d50f0eebe10
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T13:06:25.626562173Z
trivial: null
caused_by: null
created_at: 2026-11-17T13:06:25.566580356Z
---

## Goal

The window cannot be made smaller than the size every page is designed and checked at. DESIGN.md's minimum window is 1100 x 700, and the mission's checks start there, but `app/src-tauri/src/host.rs` lets the window shrink to 800 x 500. There, at 180 percent Text size, the frame is taller than the window and the status bar covers the page tabs, and in an 800 px window the status bar cuts off its cargo figure (astromech-z4vm's implementer and reviewer, 19 Nov 2026).

## Done when

- [ ] The window's minimum size is DESIGN.md's 1100 x 700, set where `host.rs` sets it today, and a saved smaller size from an earlier run is brought up to it on start. A test covers the restore of a saved 800 x 500.
- [ ] DESIGN.md's Frame says the minimum is enforced by the window, not only assumed by the layout.
- [ ] The Report says what a display smaller than 1100 x 700 gets (the window at the display's size, scrolling) and that nothing else changed.
- [ ] The Rust fmt, clippy and tests for the touched crate pass.

## Scope

The window's minimum size. The frame's behaviour below it stops mattering and is not reworked.
