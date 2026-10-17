---
schema: atelier.review
schema_version: 1
id: astromech-mm3l
title: 'Both fixes verified: claim UPDATE skips rows discarded mid-pass (attempts/backoff unchanged for normal sends); companion age now measured from arrival using the caller-supplied now. Each new test fails with its fix reverted. Merge 81ea5c0 remerge-diff empty. fmt, clippy, tests, metadata pass. Non-blocking: outbox ids are reused after a purge (no AUTOINCREMENT), so a purge plus new enqueue within one pass can still match; filed as a low follow-up bug.'
target: astromech-6xyo
identity: review-agent (6xyo fix review, sonnet)
commit: 061f43d76d357824122fa3e640a9d46e7b072706
created_at: 2026-10-17T00:11:18.217888926Z
event:
  event: approval
---

Both fixes verified: claim UPDATE skips rows discarded mid-pass (attempts/backoff unchanged for normal sends); companion age now measured from arrival using the caller-supplied now. Each new test fails with its fix reverted. Merge 81ea5c0 remerge-diff empty. fmt, clippy, tests, metadata pass. Non-blocking: outbox ids are reused after a purge (no AUTOINCREMENT), so a purge plus new enqueue within one pass can still match; filed as a low follow-up bug.
