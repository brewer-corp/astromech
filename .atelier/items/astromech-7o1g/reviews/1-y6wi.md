---
schema: atelier.review
schema_version: 3
round: 1
commit: a055c876d1b7eb7b48b23ea65ef5458c36fb5b89
reviewers:
- thread:delegated-task:command%3Amcp%3A3f85d766-962a-4fbc-95ff-3c45f2e2ffb9%3Adelegate-task%3Ab020f9cb-8d90-4eb0-a350-55858a506945
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T13:11:28.096541486Z
---

## Summary

Complexity: low. The change adds one sizing helper and one startup adjustment around the existing window builder.

Value: medium. It prevents ordinary resizing below the size the current pages are designed for, while accommodating smaller displays.

Risk of merging: medium. Incorrect constraints affect the installed window, but the change is visible and can be reverted without losing data.

The value is worth the size of the change. The small-display exception needs the correction below before merging.

I reviewed `git diff main...a055c876d1b7eb7b48b23ea65ef5458c36fb5b89`, the complete host lifecycle, `window_size.rs`, the Tauri configuration and dependencies, DESIGN.md's Frame, CONTRIBUTING.md, and the item's Done when lines and Report. The Report correctly rejects the saved-size premise: no window-state plugin or geometry persistence exists, and each new window is built at the configured opening size. Adding persistence solely to test restoration would exceed this item's scope. The builder now asks for 1100 x 700, the design documents enforcement and the small-display exception, and the page retains its existing scrolling.

I independently ran `cargo fmt --all --check`, `cargo clippy -p astromech-app --all-targets -- -D warnings`, and `cargo test -p astromech-app`; all passed. The package run passed 24 tests; one existing helper test is ignored in the normal runner and invoked by its parent test. I checked the new Tauri calls against the installed tauri 2.12.3 source and constraint behavior against tao 0.37.1's Windows implementation. These Linux checks do not compile `host.rs`, which requires the `window` feature and unavailable webkit2gtk system libraries. I did not run Windows or observe native resizing, monitor movement, or DPI changes. The finding below follows from the complete event handling and tao's constraint storage, rather than a Windows runtime capture.

## Finding 1: Recompute the minimum when the window's display or DPI changes
Recommendation: fix now

`app/src-tauri/src/host.rs:135-136` calls `fit_display` only immediately after building the window. At lines 283-285, that function permanently replaces the logical minimum with the opening display's available room. No later path recomputes it: the event handler at lines 438-451 handles destruction and shutdown, and reopening an existing window returns at line 109 without adjusting its constraints.

For example, open on a display whose work area leaves less than 1100 x 700 logical pixels, then move the window onto a display with enough room. The minimum remains the smaller opening-display value, so the commander can still shrink the window below the design minimum on the larger display. Reducing the display's scaling while the app remains open has the same problem. In the opposite direction, moving from a large display to a smaller one retains the full 1100 x 700 minimum and prevents the promised fit to the smaller work area. tao stores these minimum dimensions in `WindowState.size_constraints`; `WM_GETMINMAXINFO` reapplies them without capping them to the current monitor, and `WM_DPICHANGED` updates the scale factor without recalculating the logical minimum.

Recompute the effective minimum when the current monitor, its available work area, or DPI changes, restoring 1100 x 700 once it fits and relaxing it only while the current display requires that exception. Keep this separate from the initial size and position adjustment so moving the window does not repeatedly reset its size or position. The early return at lines 280-281 also needs to permit restoring a previously relaxed minimum. Cover a small-to-large transition and its reverse at the sizing-policy boundary.
