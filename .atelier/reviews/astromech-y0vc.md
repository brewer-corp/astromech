---
schema: atelier.review
schema_version: 2
id: astromech-y0vc
title: approval on astromech-bp23 @bb8a2493
target: astromech-bp23
identity: claude-reviewer-i8vr
commit: bb8a24938dcf485760d9dbb8ff8448347ad6fe9e
created_at: 2026-11-07T09:51:32.014263361Z
event:
  event: approval
---

Re-review of the fixes for klw0 and 1k9v (845cf3bb), the i8vr record update (bfece8c1) and the dispositions (8cd667ff, e3b17539, bb8a2493). klw0: call_window now passes Some(SHORT) for cmd == status only; every other window call still has no read timeout. The new test the_windows_status_call_gives_up_on_a_core_that_never_answers uses a listener that reads the request and never replies. It returns the client's 'waiting for the core's reply to status' error in about 10 s, and without the bound its 30 s guard would fail it. The i8vr Done when line 1, the window.rs doc and ARCHITECTURE 18.2 now say status keeps the bound. 1k9v: star_map_preload catches (ValueError, OSError), and py_compile passes. 34i3 is deferred to 96dv with a note on it. Reran: cargo test -p astromech-api control 7/7 (10.4 s), clippy -p astromech-api --all-targets -D warnings clean, fmt clean. The shell change is a doc comment only. Info: the new test costs about 10 s of real wall-clock time in cargo test. Still for the PR: Windows CI and the Release onboarding smoke (i8vr Done when lines 4 and 6).
