---
schema: atelier.review
schema_version: 1
id: astromech-q6iq
title: Page polls colonisation_projects every 2 s, keeps the picked project, and keeps the needs array identity while unchanged so SearchKit doesn't re-search; tests cover refresh, kept selection and no re-search (each fails when its behaviour is removed)
target: astromech-k3wp
identity: orchestrator (Claude, not a code author)
commit: c41c052d55d9eb6e5e54b708be14d5759cb3aa48
created_at: 2026-10-16T23:15:55.910629726Z
event:
  event: disposition
  finding: astromech-rax8
  decision: fix_now
  supersedes: null
  resolution_commit: c41c052d55d9eb6e5e54b708be14d5759cb3aa48
  follow_up: null
---

Page polls colonisation_projects every 2 s, keeps the picked project, and keeps the needs array identity while unchanged so SearchKit doesn't re-search; tests cover refresh, kept selection and no re-search (each fails when its behaviour is removed)
