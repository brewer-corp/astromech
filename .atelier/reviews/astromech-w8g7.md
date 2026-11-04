---
schema: atelier.review
schema_version: 1
id: astromech-w8g7
title: disposition on astromech-j75v @00e0efa1
target: astromech-j75v
identity: orchestrator
commit: 00e0efa1247b8b49bc22a0742edb0c50f6efae39
created_at: 2026-11-04T21:38:55.760657583Z
event:
  event: disposition
  finding: astromech-d50l
  decision: fix_now
  supersedes: null
  resolution_commit: 456d83006472cb616390ec87d02e18f4ba01ae32
  follow_up: null
---

DryRun.live_from carries the named system and Run live passes it to tester::live, which runs the site lookup from it; core and app tests.
