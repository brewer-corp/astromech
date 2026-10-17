---
schema: atelier.review
schema_version: 1
id: astromech-ida5
title: Test config now sets game.journal_dir to a missing temp folder (first-run case); reproduced the timeout with USERPROFILE pointing at a fake home with journals, passes after the fix
target: astromech-5t34
identity: orchestrator (Claude, not a code author)
commit: c3ed7ec6cb4774c59f461dc2327cf0a73f88e489
created_at: 2026-10-17T00:10:43.250599335Z
event:
  event: disposition
  finding: astromech-pex1
  decision: fix_now
  supersedes: null
  resolution_commit: c3ed7ec6cb4774c59f461dc2327cf0a73f88e489
  follow_up: null
---

Test config now sets game.journal_dir to a missing temp folder (first-run case); reproduced the timeout with USERPROFILE pointing at a fake home with journals, passes after the fix
