---
schema: atelier.item
schema_version: 3
id: astromech-7o1g
title: The window can shrink to 800 x 500, below the 1100 x 700 the design is built for
status: approved
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

- [x] The window's minimum size is DESIGN.md's 1100 x 700, set where `host.rs` sets it today, and a saved smaller size from an earlier run is brought up to it on start. A test covers the restore of a saved 800 x 500.
- [x] DESIGN.md's Frame says the minimum is enforced by the window, not only assumed by the layout.
- [x] The Report says what a display smaller than 1100 x 700 gets (the window at the display's size, scrolling) and that nothing else changed.
- [x] The Rust fmt, clippy and tests for the touched crate pass.

## Scope

The window's minimum size. The frame's behaviour below it stops mattering and is not reworked.

## Report

**The first Done when line's premise does not hold: no size is saved between runs, so there is nothing to restore.** `host.rs` sets the window's size in one place, the builder in `open_window`; the app has no window-state plugin, `tauri.conf.json` declares no windows, and the window is destroyed on close and built again at 1200 x 800 every time. So no test can restore a saved 800 x 500, and I did not add size saving to make one possible (out of Scope). The test covers the case that does exist instead: what the minimum becomes on a display with less room.

**What changed** (`56b00abe`):

- The minimum is DESIGN.md's 1100 x 700, set in the same builder call (`min_inner_size`), from `app/src-tauri/src/window_size.rs` (`MINIMUM`, `START`).
- A display smaller than 1100 x 700 gets the window at the display's size, and the page scrolls. tao sets the minimum on Windows unconditionally (`WM_GETMINMAXINFO` in tao 0.37.1's `event_loop.rs` writes `ptMinTrackSize` with no cap to the monitor), so a bare 1100 x 700 would hold the window partly off screen on, for example, 1366 x 768 at 100 percent (about 697 px of inner height under a 40 px taskbar) or 1920 x 1080 at 150 percent (1280 x 720 logical, about 657 px). After building, `fit_display` in `host.rs` reads the window's monitor work area, takes off the window's frame (outer minus inner size), and where that room is less than the 1200 x 800 it opens at, brings the minimum and size down to the room (`window_size::fit`) and moves the window to the work area's corner. On a display with the room, nothing changes. The same overflow of the 1200 x 800 opening size already happened on those displays before this change.
- DESIGN.md's Frame says the window enforces the minimum, and only a too-small display gets less; the 800 x 500 paragraph no longer says the window allows it. The 800 x 500 header test in `App.browser.test.ts` stays, since a small display can still get a window that size; only its comment changed.
- Nothing else changed: the frame's layout below the minimum is not reworked.

**What I ran and what I only read.** `cargo fmt --all --check`, `cargo clippy -p astromech-app --all-targets -- -D warnings` and `cargo test -p astromech-app --bin astromech-core` (the two `window_size` tests) pass on Linux, without the `window` feature. `host.rs` is behind the `window` feature, which needs webkit2gtk, not installed here, so `fit_display` was not compiled locally; CI's Windows job builds and clippies it (`--features window`). Its Tauri calls (`current_monitor`, `work_area`, `outer_size`, `inner_size`, `set_min_size`, `set_size`, `set_position`) were checked against tauri 2.12.3's source. Nothing was run on Windows.

**Round 2** (`f339c85b`): the minimum now follows the display the window is on. On `WindowEvent::Moved` and `ScaleFactorChanged` for the window, `Host::refit_minimum` works out the room again and sets the minimum only when `window_size::Minimum::refit` says it changed (setting a minimum makes tao re-check the window's size, so not on every move of a drag). Moving to a display with the room gives 1100 x 700 back; moving to a smaller one relaxes it. Only the minimum changes then: the window keeps its size and place, and the opening size and corner move happen once, at build. A test covers small to large and back. Same limits as before: not compiled with the `window` feature here, not run on Windows.

**Limit.** On Windows 10 and 11, `outer_size` includes the invisible resize borders (about 7 px a side), so on a small display the window comes out a few px smaller than the room it could take. Harmless, and not worth a Win32 call.

### Responses

1-y6wi.1: fixed in f339c85b
