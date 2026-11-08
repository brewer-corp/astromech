---
schema: atelier.review
schema_version: 2
id: astromech-pdiw
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:08.612214434Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:17 the reload and rebuild are true (astromech-v3u4 report; osyy and the probe's PASS 6, 'reloading the page' and 'rebuilt at keybinds'), but the heading 'A window crash leaves astromech running' was also true in alpha.4, where the window was its own process. What is new is that the window comes back by itself. Fix: retitle it 'The window recovers from a crash' and lead with that. CI showed the tray and control API stay up; voice and the kill switch during a window crash are by design, not tested.
