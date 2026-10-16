---
schema: atelier.review
schema_version: 1
id: astromech-twee
title: Live run waits up to 5 s for Elite to come to the front (Policy::wait_foreground) with a visible countdown; tested with another window active.
target: astromech-q318
identity: orchestrator (Claude, not a code author)
commit: c6165b53e032d49bf459e2a886f8071efea0d908
created_at: 2026-10-16T19:07:14.740317635Z
event:
  event: disposition
  finding: astromech-2h7b
  decision: fix_now
  supersedes: null
  resolution_commit: 327fa7f84229de662b1b82a2fe39871e2fa691c0
  follow_up: null
---

Live run waits up to 5 s for Elite to come to the front (Policy::wait_foreground) with a visible countdown; tested with another window active.
