---
schema: atelier.review
schema_version: 1
id: astromech-m2wp
title: Fixture ends with a Docked 20 minutes after the last transfer is due; the test passes with the Utc::now settle removed.
target: astromech-hnar
identity: orchestrator
commit: e17f3edd06a6054c8190858fd159d9815c238ab7
created_at: 2026-10-17T17:30:52.266570189Z
event:
  event: disposition
  finding: astromech-71w2
  decision: fix_now
  supersedes: null
  resolution_commit: e17f3edd06a6054c8190858fd159d9815c238ab7
  follow_up: null
---

Fixture ends with a Docked 20 minutes after the last transfer is due; the test passes with the Utc::now settle removed.
