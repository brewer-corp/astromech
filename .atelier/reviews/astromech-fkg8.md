---
schema: atelier.review
schema_version: 1
id: astromech-fkg8
title: 'R3 low: cache.rs init() drops the old-schema table non-transactionally; if core and app window open a v0.1.5 file at once, the second DROP may fail and that process falls back to an in-memory cache for the run (logged, no user data lost). Wrap check+drop+create in one transaction.'
target: astromech-zs2q
identity: review-agent (sonnet, independent)
commit: 744a26a39adb67c24914d76aa9ebe47c1843c2e0
created_at: 2026-10-17T06:00:43.969255417Z
event:
  event: finding
  recommendation: defer
---

R3 low: cache.rs init() drops the old-schema table non-transactionally; if core and app window open a v0.1.5 file at once, the second DROP may fail and that process falls back to an in-memory cache for the run (logged, no user data lost). Wrap check+drop+create in one transaction.
