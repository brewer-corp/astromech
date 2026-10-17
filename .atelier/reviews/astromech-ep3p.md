---
schema: atelier.review
schema_version: 1
id: astromech-ep3p
title: 'concurrent_first_open_succeeds uses Connection::open with a 10 s busy_timeout rather than Store::open (rusqlite''s 5 s default). I reran it through Store::open 20 times: all passed; unmodified 20/20 passed; without enable_wal it failed 4 of 5 with ''database is locked''. The test is sound; it just doesn''t use the production entry point.'
target: astromech-n1ij
identity: review-agent (opus, independent)
commit: df2c4fb438a4f201a3a05fc02770c5536db43657
created_at: 2026-10-17T05:05:44.642669025Z
event:
  event: finding
  recommendation: no_action
---

concurrent_first_open_succeeds uses Connection::open with a 10 s busy_timeout rather than Store::open (rusqlite's 5 s default). I reran it through Store::open 20 times: all passed; unmodified 20/20 passed; without enable_wal it failed 4 of 5 with 'database is locked'. The test is sound; it just doesn't use the production entry point.
