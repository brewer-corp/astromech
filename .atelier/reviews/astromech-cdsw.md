---
schema: atelier.review
schema_version: 1
id: astromech-cdsw
title: 'N1: core_call is a sync Tauri command; a live run blocks the main thread up to ~7 s, including the window''s kill switch.'
target: astromech-q318
identity: review-agent (q318 round 2)
commit: 5d81e28c1154ac0ed92d64dede56a8e2a775ec26
created_at: 2026-10-16T19:22:48.361421266Z
event:
  event: finding
  recommendation: fix_now
---

N1: core_call is a sync Tauri command; a live run blocks the main thread up to ~7 s, including the window's kill switch.
