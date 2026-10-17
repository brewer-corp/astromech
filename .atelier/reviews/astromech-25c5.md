---
schema: atelier.review
schema_version: 1
id: astromech-25c5
title: 'Integration with astromech-dt5h (when rebasing on main after it lands): dt5h''s SystemViewCache caches a finished SystemView and main.rs then only adds landings. Both branches rewrite exploration_system; a resolution that keeps cache.view() + add_landings silently drops the earlier-visit bodies this issue adds. The cache must hold the live SystemFold (from the two newest journals) and History::system_view(fold) must run each poll, so history changes still show.'
target: astromech-0n4h
identity: review-agent (opus, independent)
commit: e1a1d87ea2d901d8c55dd406f77c768b1fe467a0
created_at: 2026-10-17T08:00:42.101997588Z
event:
  event: finding
  recommendation: fix_now
---

Integration with astromech-dt5h (when rebasing on main after it lands): dt5h's SystemViewCache caches a finished SystemView and main.rs then only adds landings. Both branches rewrite exploration_system; a resolution that keeps cache.view() + add_landings silently drops the earlier-visit bodies this issue adds. The cache must hold the live SystemFold (from the two newest journals) and History::system_view(fold) must run each poll, so history changes still show.
