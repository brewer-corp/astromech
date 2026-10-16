---
schema: atelier.review
schema_version: 1
id: astromech-ljdf
title: 'Reviewed fix commits 8214290 (7jv7), 47a8a0c (tr4g), 2b56c35 (5kdt), 3065314 (u76o). 7jv7: lookup runs on its own thread; the new test would fail if the lookup were inline (handle would return Spoken after the 20 s gate, not LookingUp), asserts gear handled within 2 s while the lookup is unfinished; Speaker queue is FIFO so speech does not interleave mid-utterance; threads are detached and die with the process. tr4g: request shape and dedupe by id64 correct and tested; one low deferrable finding (second request failure fails the whole lookup). 5kdt and u76o correct. fmt, clippy -D warnings, cargo test --workspace, check-deps, cargo metadata --locked --offline, pnpm install/check/test all pass.'
target: astromech-sfys
identity: review-agent (sfys fix review, sonnet)
commit: e4a8266d52ce3118d507b47dc12e4b51aafd200e
created_at: 2026-10-16T22:40:45.196193967Z
event:
  event: approval
---

Reviewed fix commits 8214290 (7jv7), 47a8a0c (tr4g), 2b56c35 (5kdt), 3065314 (u76o). 7jv7: lookup runs on its own thread; the new test would fail if the lookup were inline (handle would return Spoken after the 20 s gate, not LookingUp), asserts gear handled within 2 s while the lookup is unfinished; Speaker queue is FIFO so speech does not interleave mid-utterance; threads are detached and die with the process. tr4g: request shape and dedupe by id64 correct and tested; one low deferrable finding (second request failure fails the whole lookup). 5kdt and u76o correct. fmt, clippy -D warnings, cargo test --workspace, check-deps, cargo metadata --locked --offline, pnpm install/check/test all pass.
