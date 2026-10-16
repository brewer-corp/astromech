---
schema: atelier.review
schema_version: 1
id: astromech-xxnw
title: 'R1: materials_inventory is a sync Tauri command, so it runs on the main thread; with the new key it recomputes about once a second during play (~157 ms per 8 MB journal), stuttering the window.'
target: astromech-txzq
identity: review-agent (txzq round 1)
commit: 28333d829e7b991169bb03084f62e1b21a86e8fc
created_at: 2026-10-16T18:44:29.969744024Z
event:
  event: finding
  recommendation: fix_now
---

R1: materials_inventory is a sync Tauri command, so it runs on the main thread; with the new key it recomputes about once a second during play (~157 ms per 8 MB journal), stuttering the window.
