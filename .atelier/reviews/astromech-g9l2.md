---
schema: atelier.review
schema_version: 1
id: astromech-g9l2
title: the unbounded try_recv drain can still starve on_wake under a gapless burst
target: astromech-mosx
identity: mosx-reviewer-1
commit: bb7ccf3f1678f5ea8b8c6c792ff70ded73a4dac9
created_at: 2026-10-17T18:43:05.434006993Z
event:
  event: finding
  recommendation: fix_now
---

the unbounded try_recv drain can still starve on_wake under a gapless burst
