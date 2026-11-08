---
schema: atelier.review
schema_version: 2
id: astromech-21ds
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:09.385560831Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:22 the Status.json bullet is worth keeping, but it explains the mechanism before the effect. Fix: lead with what the player sees, for example 'astromech no longer misses that you opened the galaxy map or a panel', then say it re-checks the file a few times a second (107e5c5d, every 250 ms when idle).
