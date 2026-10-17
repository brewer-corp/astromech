---
schema: atelier.review
schema_version: 1
id: astromech-hpbr
title: enable_wal is duplicated from astromech-store (same 5s bound, 5ms sleep, DatabaseBusy only); crate cannot depend on store, so the copy is acceptable and noted in the doc comment. Retry is bounded and matches store.
target: astromech-qtiw
identity: review-agent (sonnet, independent)
commit: 625ec2ac2b8eecf171f8ff5c65faebd193e1a204
created_at: 2026-10-17T07:01:36.697900262Z
event:
  event: finding
  recommendation: no_action
---

enable_wal is duplicated from astromech-store (same 5s bound, 5ms sleep, DatabaseBusy only); crate cannot depend on store, so the copy is acceptable and noted in the doc comment. Retry is bounded and matches store.
