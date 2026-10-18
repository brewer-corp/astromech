---
schema: atelier.review
schema_version: 1
id: astromech-h9kb
title: 'R1 medium lib.rs normalise: ''i ll'' joined to ''ill'' (one letter from ''all'') so the uncapped path scores ''I''ll stop'' 0.833, above MISHEARD_CAP 0.80 and 0.017 below EXECUTE; the documented cap does not hold. Fix: join to a token not close to ''all'', and assert score <= MISHEARD_CAP in the test.'
target: astromech-y296
identity: review-agent (opus, independent, y296 round 1)
commit: c1fc656542f10d4e802972285afc3ab2aaa63677
created_at: 2026-10-17T23:59:03.454518985Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium lib.rs normalise: 'i ll' joined to 'ill' (one letter from 'all') so the uncapped path scores 'I'll stop' 0.833, above MISHEARD_CAP 0.80 and 0.017 below EXECUTE; the documented cap does not hold. Fix: join to a token not close to 'all', and assert score <= MISHEARD_CAP in the test.
