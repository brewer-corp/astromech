---
schema: atelier.review
schema_version: 1
id: astromech-ompq
title: 'R1 medium lib.rs:873: two-content-word phrases where every content word is a sound-alike still execute (''rice care'' -> raise gear, ''drape care'', ''chalk maeby'' -> galaxy map, ''had cambodia'' -> hud combat): all 276 generated inputs execute. Fix: track ''backed'' (a content word or exact slot heard on its spelling) in the table; lone = sound && !backed.'
target: astromech-vkzj
identity: review-agent (opus, independent, vkzj round 2)
commit: 74578242b1baa792160e1867af392b678519075b
created_at: 2026-10-18T03:21:28.076505153Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium lib.rs:873: two-content-word phrases where every content word is a sound-alike still execute ('rice care' -> raise gear, 'drape care', 'chalk maeby' -> galaxy map, 'had cambodia' -> hud combat): all 276 generated inputs execute. Fix: track 'backed' (a content word or exact slot heard on its spelling) in the table; lone = sound && !backed.
