---
schema: atelier.review
schema_version: 2
id: astromech-p4v3
title: approval on astromech-bp23 @af359b90
target: astromech-bp23
identity: claude-reviewer-ro7a
commit: af359b90200a62ce0a93bcaa4fda1cb3c6bfcf75
created_at: 2026-11-07T11:19:26.057593891Z
event:
  event: approval
---

Re-review of 3e64e50c (fixes for zpws, kp81, nhbc), be708cea/4fa59a17/af359b90 (nq3c noted on vucf, ro7a placement accepted, dispositions). zpws: vucf line 2 now requires every window command called, off-Windows autostart/mic_check refusing on their own (true: autostart.rs:130, calibration.rs:323); Want::NotCalled documented as ro7a-only. kp81: ARCHITECTURE 18.2 says advances, vucf completes. nhbc: the status-line test takes CONFIG_EDITS. cargo test -p astromech-core --lib 220 passed, clippy -p astromech-core -D warnings and fmt clean at HEAD; earlier at 36531f8c cargo test --workspace --locked 1302 passed and strace of the test showed only 127.0.0.1 and no writes outside temp. Branch against bp23 Done when: generate_handler![call] only, no other tauri::command or .manage, shell deps trimmed; dispatch behind catch_unwind on per-connection threads; ts-rs bindings test in the workspace run; ro7a test plus typed call() (transport.ts the app's only invoke); State::release_idle on the reaper. Unmet only on the PR: Release onboarding smoke over every page and Windows CI shell build/clippy; the new test is also unseen on Windows CI.
