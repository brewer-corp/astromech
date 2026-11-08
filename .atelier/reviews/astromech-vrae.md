---
schema: atelier.review
schema_version: 2
id: astromech-vrae
title: finding on astromech-h95t @1890eaaa
target: astromech-h95t
identity: claude-reviewer-h95t
commit: 1890eaaa3080cc66ebcf6f63edc52795874739d6
created_at: 2026-11-08T16:03:05.969976368Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:13 'and now builds no window at all until you open it' still reads as a change, but alpha.4 also built none at login: the Run key started astromech-core.exe --background, which skips the window, and the shell was not started (v1.0.0-alpha.4:docs/V1-DESIGN.md 7.10, 7.1). A player sees no difference. Fix: drop the bullet (33df's first option), or write 'As in alpha.4, at login astromech starts quietly in the tray and builds no window until you open it.'
