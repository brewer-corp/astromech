---
schema: atelier.review
schema_version: 1
id: astromech-s11d
title: disposition on astromech-7c7c @d396613b
target: astromech-7c7c
identity: orchestrator
commit: d396613bd8a57b424795f9733cfda5b2e1e314f9
created_at: 2026-11-04T21:11:33.587024201Z
event:
  event: disposition
  finding: astromech-sfpa
  decision: fix_now
  supersedes: null
  resolution_commit: d396613bd8a57b424795f9733cfda5b2e1e314f9
  follow_up: null
---

announce() now waits 100 ms (a macrotask past paint) between emptying and filling; a newer call wins
