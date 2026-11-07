---
schema: atelier.review
schema_version: 2
id: astromech-m6f9
title: finding on astromech-y6d6 @9da11270
target: astromech-y6d6
identity: claude-reviewer-xz7i
commit: 9da11270e69ff3ee7e11679b161a4b132e39ae26
created_at: 2026-11-07T22:10:53.347749313Z
event:
  event: finding
  recommendation: defer
---

low: scripts/webview_cost.py:446-453 (App.tray) and :102-108 (tray_verdict). 'fallback' means only that ui_state.tray is false and 'tray icon ready' was logged at some point in the run. The real app's ui_state has no tray_built or tray_rect (host.rs:344-350), so Err from TrayIcon::rect, or a tray that had a rectangle and lost it mid-run (rect at the windowless check, then fallback after the browser crash), reads DEGRADED and doesn't fail. Shown in scratch: crashes[1].tray='fallback' with every other reading 'rect' -> DEGRADED, failed() False. The decision note's case (a session with no notification area) reads fallback from the start. Suggested: fail when a run with a 'rect' reading later reads 'fallback', and keep DEGRADED for all-fallback runs.
