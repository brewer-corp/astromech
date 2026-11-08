---
schema: atelier.review
schema_version: 2
id: astromech-33df
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:07.408820521Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:13 'Start with Windows shows the tray only' is unchanged from alpha.4, whose Settings, Startup already said it 'starts quietly in the tray' with no window (v1.0.0-alpha.4:docs/guide/settings.md). Fix: drop the bullet, or fold 'as before' into the one-app bullet.
