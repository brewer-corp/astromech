---
schema: atelier.review
schema_version: 2
id: astromech-ah78
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:06.836311165Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/releases/v1.0.0-alpha.5.md:3,12 present 'closing the window gives its memory back' as new, but alpha.4 already did it: its window process exited on close and freed WebView2 (v1.0.0-alpha.4:app/src-tauri/src/main.rs:11, alpha.4 V1-DESIGN 7.1), and push-to-talk and the kill switch already ran in the tray. The lead tells alpha.4 players about a gain they won't see. Fix: lead with what does change (one astromech-core.exe in Task Manager, no separate background program for the window to lose, so the old 'background process is not running' message is gone; old shortcuts forward), and say closing the window still frees its memory, as before.
