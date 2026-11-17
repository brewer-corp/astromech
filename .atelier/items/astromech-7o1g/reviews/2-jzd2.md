---
schema: atelier.review
schema_version: 3
round: 2
commit: 891a13acbf6dfac771b2eb4dccbf95b79e62ee60
reviewers:
- thread:delegated-task:command%3Amcp%3A3f85d766-962a-4fbc-95ff-3c45f2e2ffb9%3Adelegate-task%3A95adc06e-b469-4de8-8511-09303ccd7178
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T13:14:55.896714547Z
---

## Summary

Complexity: medium. The change coordinates the native window builder, display measurements and a cached minimum updated by window events; it remains small enough to review in one sitting.

Value: medium. It enforces the minimum used by the current pages while accommodating displays that cannot fit it.

Risk of merging: medium. Incorrect sizing affects the installed window, but the result is visible and the change can be reverted without losing data.

The value is worth the change. I found no remaining fix-now problem.

I reviewed `git diff main...891a13acbf6dfac771b2eb4dccbf95b79e62ee60` and the changes since round 1, the complete host lifecycle, `window_size.rs`, Tauri configuration and dependencies, DESIGN.md's Frame, the changed browser-test comment, CONTRIBUTING.md, and the item's Done when lines and Report. The saved-size premise is indeed absent: no geometry persistence or window-state plugin exists, the configuration declares no windows, and reopening a destroyed window builds it at 1200 x 800. Adding persistence solely to exercise restoration would exceed the scope. The builder now requests 1100 x 700; the startup adjustment caps the minimum and opening size to the display's available inner room. DESIGN.md documents enforcement and the small-display exception. The existing page scrolling and layout are unchanged.

For the carried finding, I traced `Moved` and `ScaleFactorChanged` through Tauri 2.12.3 and tauri-runtime-wry 2.12.1, and checked the monitor queries and constraint setter in tao 0.37.1's Windows implementation. The event handler recalculates the effective minimum for the current monitor. The cached value prevents setting identical constraints on every move, and its lock is released before the native setter. Startup alone changes the opening size and position. The policy test exercises a smaller display, an unchanged minimum, restoration to 1100 x 700, and relaxation again.

I independently ran `cargo fmt --all --check`, `cargo clippy -p astromech-app --all-targets -- -D warnings`, and `cargo test -p astromech-app`; all passed. The package run passed 25 tests, including the three window-sizing tests. One existing helper test is ignored by the normal runner and invoked by its parent test. These Linux checks do not compile `host.rs`: the `window` feature needs unavailable webkit2gtk/GTK system libraries. I checked the added APIs and event patterns against the installed dependency sources and confirmed that the Windows CI job builds and clippies the window feature, but did not run that job or the Windows app. Native resizing, monitor movement and DPI changes were therefore not observed at runtime.

## Finding 1: Recompute the minimum when the window's display or DPI changes
Recommendation: fix now
Was: 1-y6wi.1
Response: fixed in f339c85b
Now: fixed

`Host::refit_minimum` now runs for this window's `Moved` and `ScaleFactorChanged` events as well as during startup fitting. It reads the current monitor's work area and frame size, and `Minimum::refit` returns the full design minimum again when that room permits it. The former startup early return no longer blocks a later restoration. The new transition test passes, including restoration and the reverse transition; the event path does not invoke the startup size or corner adjustment. This addresses the reported monitor/DPI transition defect.
