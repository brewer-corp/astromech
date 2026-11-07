---
schema: atelier.review
schema_version: 2
id: astromech-trln
title: finding on astromech-y6d6 @26afef19
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 26afef19d845b864358994d291ec64c98ed410ca
created_at: 2026-11-07T17:57:43.619053362Z
event:
  event: finding
  recommendation: no_action
---

info: app/src-tauri/src/host.rs:239-256, 144-149. If rebuild_window's 5 s wait for the dead window to go times out, open_window builds the new window and start_input replaces input_stop; a late Destroyed for the old window then calls window_destroyed, which takes and stops the new window's press listener (press-to-bind and the key tester go quiet until the next reopen). In the normal case Tauri removes the window and runs our Destroyed callback in the same event dispatch (tauri-2.12.3 app.rs on_event_loop_event), before the rebuild's build message is handled, so this needs the timeout path. Could key input_stop by window instance; not needed now.
