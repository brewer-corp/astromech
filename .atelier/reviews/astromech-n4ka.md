---
schema: atelier.review
schema_version: 1
id: astromech-n4ka
title: 'Polls sequenced with a request counter (test: a slow older answer after a newer one is ignored; fails without the counter); shell cache key compares Cargo.json bytes instead of size+mtime. Hidden-err display left as directed.'
target: astromech-k3wp
identity: orchestrator (Claude, not a code author)
commit: f219f91bfddf01b3ccbe1d7a4046ff179bae2a31
created_at: 2026-10-16T23:20:05.799922173Z
event:
  event: disposition
  finding: astromech-t8cf
  decision: fix_now
  supersedes: null
  resolution_commit: f219f91bfddf01b3ccbe1d7a4046ff179bae2a31
  follow_up: null
---

Polls sequenced with a request counter (test: a slow older answer after a newer one is ignored; fails without the counter); shell cache key compares Cargo.json bytes instead of size+mtime. Hidden-err display left as directed.
