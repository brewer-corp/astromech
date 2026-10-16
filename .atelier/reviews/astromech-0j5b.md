---
schema: atelier.review
schema_version: 1
id: astromech-0j5b
title: from_journal_dir takes the ship's hold from Cargo.json after the replay (SRV or unreadable file keeps the replayed hold); shell cache key includes Cargo.json; test cargo_json_overrides_the_replayed_hold fails without the override
target: astromech-k3wp
identity: orchestrator (Claude, not a code author)
commit: c41c052d55d9eb6e5e54b708be14d5759cb3aa48
created_at: 2026-10-16T23:15:56.016513755Z
event:
  event: disposition
  finding: astromech-vok2
  decision: fix_now
  supersedes: null
  resolution_commit: c41c052d55d9eb6e5e54b708be14d5759cb3aa48
  follow_up: null
---

from_journal_dir takes the ship's hold from Cargo.json after the replay (SRV or unreadable file keeps the replayed hold); shell cache key includes Cargo.json; test cargo_json_overrides_the_replayed_hold fails without the override
