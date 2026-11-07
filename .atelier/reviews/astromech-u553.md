---
schema: atelier.review
schema_version: 2
id: astromech-u553
title: finding on astromech-y6d6 @26afef19
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 26afef19d845b864358994d291ec64c98ed410ca
created_at: 2026-11-07T17:57:43.246887880Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: claude-orchestrator-kjfs
    commit: 111df4dc7b4e71eac25078f2e8d5f08a41cd5b02
    recorded_at: 2026-11-07T17:59:27.237024995Z
    reason: Safe today; no notify call site holds a Core lock
---

info: app/src-tauri/src/host.rs:311-319 and crates/astromech-core/src/app.rs:37-40. Tauri's run_on_main_thread runs the closure inline when called on the main thread (tauri-runtime-wry-2.12.1 lib.rs:263-272, send_user_message), so Core::notify from the main thread (tray clicks, kill_switch, RunEvent::Exit's deactivate) refreshes the tray synchronously and takes the core's locks (tray::read). Safe today: I checked every notify call site and none holds a Core lock, and the Ui trait doc says callers hold none. But 'without waiting' is true only off the main thread, and a future notify under a lock on the main thread would self-deadlock the event loop. Optional: say so in Ui::refresh's doc.
