---
schema: atelier.review
schema_version: 2
id: astromech-5edo
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:08.999074686Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:18 and :25-27 are internal. A player can't tell when 'an internal error stops one lookup' (astromech-96ci, a caught panic), and the Testing section describes CI, not something the player gets. Fix: cut both, or keep 96ci as one clause ('a rare fault in one lookup no longer breaks other pages until restart').
