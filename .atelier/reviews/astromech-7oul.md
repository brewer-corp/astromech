---
schema: atelier.review
schema_version: 1
id: astromech-7oul
title: Startup follow_history now runs under config_lock, so a set_folders served meanwhile isn't re-pointed back
target: astromech-5t34
identity: orchestrator (Claude, not a code author)
commit: c3ed7ec6cb4774c59f461dc2327cf0a73f88e489
created_at: 2026-10-17T00:10:43.373016306Z
event:
  event: disposition
  finding: astromech-t282
  decision: fix_now
  supersedes: null
  resolution_commit: c3ed7ec6cb4774c59f461dc2327cf0a73f88e489
  follow_up: null
---

Startup follow_history now runs under config_lock, so a set_folders served meanwhile isn't re-pointed back
