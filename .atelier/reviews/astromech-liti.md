---
schema: atelier.review
schema_version: 1
id: astromech-liti
title: 'Approved at 2b8bc7d. Read the diff and the issue. The v2 migration rebuilds outbox with INTEGER PRIMARY KEY AUTOINCREMENT inside BEGIN/COMMIT, copying every column and id, so rows are kept and sqlite_sequence starts at the highest copied id; already-migrated files skip it via user_version, and a crash mid-batch rolls back. deliver_due''s claim-by-id is now safe because a purged id is never handed out again. Ran cargo test -p astromech-eddn: 110 pass, including new_messages_never_take_a_purged_messages_id and migrating_a_v1_outbox_keeps_its_rows. Confirmed the first test exercises the bug: on an archive copy with the v2 migration removed it fails, posting a, b, c instead of a. The migration test checks a v1 row survives and the next id after a purge exceeds 7. Full gates taken from the author''s report, not rerun. R1 (non-atomic version check, a narrow concurrent-first-open race) is deferred, not blocking.'
target: astromech-8ozh
identity: review-agent (opus, independent)
commit: 2b8bc7d84e35fe62a6797df2e98da1e7c47aae55
created_at: 2026-10-17T04:55:06.368564279Z
event:
  event: approval
---

Approved at 2b8bc7d. Read the diff and the issue. The v2 migration rebuilds outbox with INTEGER PRIMARY KEY AUTOINCREMENT inside BEGIN/COMMIT, copying every column and id, so rows are kept and sqlite_sequence starts at the highest copied id; already-migrated files skip it via user_version, and a crash mid-batch rolls back. deliver_due's claim-by-id is now safe because a purged id is never handed out again. Ran cargo test -p astromech-eddn: 110 pass, including new_messages_never_take_a_purged_messages_id and migrating_a_v1_outbox_keeps_its_rows. Confirmed the first test exercises the bug: on an archive copy with the v2 migration removed it fails, posting a, b, c instead of a. The migration test checks a v1 row survives and the next id after a purge exceeds 7. Full gates taken from the author's report, not rerun. R1 (non-atomic version check, a narrow concurrent-first-open race) is deferred, not blocking.
