---
schema: atelier.review
schema_version: 1
id: astromech-9t68
title: 'Polls no longer overlap: a tick skips while one is in flight, so a slow answer always applies (data or error) and the shell never runs overlapping cold replays; the request counter is removed. Tests: a slow first poll that is the only answer shows, with no second request meanwhile, and polling resumes; a slow failure as the only answer shows its error. Both fail on the previous drop-superseded behaviour.'
target: astromech-k3wp
identity: orchestrator (Claude, not a code author)
commit: 2f214f574681234fd0de00b760b6c2f74030528c
created_at: 2026-10-16T23:23:22.616931798Z
event:
  event: disposition
  finding: astromech-x81v
  decision: fix_now
  supersedes: null
  resolution_commit: 2f214f574681234fd0de00b760b6c2f74030528c
  follow_up: null
---

Polls no longer overlap: a tick skips while one is in flight, so a slow answer always applies (data or error) and the shell never runs overlapping cold replays; the request counter is removed. Tests: a slow first poll that is the only answer shows, with no second request meanwhile, and polling resumes; a slow failure as the only answer shows its error. Both fail on the previous drop-superseded behaviour.
