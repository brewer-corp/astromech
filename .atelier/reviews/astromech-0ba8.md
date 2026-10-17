---
schema: atelier.review
schema_version: 1
id: astromech-0ba8
title: 'SystemViewCache::view lists the journals twice (signature, then from_journal_dir), so a file added or grown between the two stores a newer view under an older signature. The next poll sees a different signature and re-parses: one extra parse, never a stale answer. No action.'
target: astromech-dt5h
identity: review-agent (opus, independent)
commit: 797432034fbbec3d2aa0eaa7a812437b9455afe2
created_at: 2026-10-17T08:00:13.764764757Z
event:
  event: finding
  recommendation: no_action
---

SystemViewCache::view lists the journals twice (signature, then from_journal_dir), so a file added or grown between the two stores a newer view under an older signature. The next poll sees a different signature and re-parses: one extra parse, never a stale answer. No action.
