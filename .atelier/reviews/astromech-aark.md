---
schema: atelier.review
schema_version: 2
id: astromech-aark
title: finding on astromech-h95t @1890eaaa
target: astromech-h95t
identity: claude-reviewer-h95t
commit: 1890eaaa3080cc66ebcf6f63edc52795874739d6
created_at: 2026-11-08T16:03:06.361104318Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:21 'Opening the galaxy map, a panel or the landing gear' says the gear is opened, and the heading 'notices what the game has open' doesn't cover the gear. Fix: 'Opening the galaxy map or a panel, or lowering the landing gear, is no longer missed now and then', and a heading such as 'astromech no longer misses a change in the game's status'.
